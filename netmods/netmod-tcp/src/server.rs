//! TCP incoming connection server

use crate::{IoPair, LinkType, Mode, Packet, PacketBuilder, PeerState, Result, Routes, SourceAddr};
use async_std::{
    io::prelude::*,
    net::{SocketAddr, TcpListener, TcpStream},
    stream::StreamExt,
    sync::{Arc, RwLock},
    task,
};
use netmod::{Frame, Target};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub(crate) type LockedStream = Arc<RwLock<Option<TcpStream>>>;

fn locked_stream(s: TcpStream) -> LockedStream {
    Arc::new(RwLock::new(Some(s)))
}

/// The listening server part of the tcp driver
pub(crate) struct Server {
    alive: Arc<AtomicBool>,
    inner: TcpListener,
    routes: Arc<Routes>,
    mode: Mode,
    _port: u16,
    incoming: IoPair<(Frame, usize)>,
}

impl Server {
    /// Create a new tcp listening server, without running it
    pub(crate) async fn new(
        routes: Arc<Routes>,
        bind: SocketAddr,
        _port: u16,
        mode: Mode,
    ) -> Result<Arc<Self>> {
        Ok(TcpListener::bind(bind).await.map(|inner| {
            Arc::new(Self {
                alive: Arc::new(true.into()),
                incoming: IoPair::default(),
                inner,
                routes,
                _port,
                mode,
            })
        })?)
    }

    fn alive(self: &Arc<Self>) -> bool {
        self.alive.load(Ordering::Relaxed)
    }

    pub(crate) fn mode(&self) -> Mode {
        self.mode.clone()
    }

    /// Shut down the listening server
    pub(crate) fn stop(self: &Arc<Self>) {
        self.alive.fetch_and(false, Ordering::Relaxed);
    }

    /// Get the next available frame
    pub(crate) async fn next(self: &Arc<Self>) -> (Frame, Target) {
        self.incoming
            .rx
            .recv()
            .await
            .map(|(f, t)| (f, Target::Single(t as u16)))
            .unwrap()
    }

    /// Spawn a handler task for incoming connections
    pub(crate) fn run(self: &Arc<Self>) {
        let s = Arc::clone(self);
        task::spawn(async move {
            let mut inc = s.inner.incoming();
            info!(
                "Listening on {:?} for incoming connections",
                s.inner.local_addr()
            );

            // For each connection, spawn a new worker task
            while let Some(Ok(stream)) = inc.next().await {
                if !s.alive() {
                    break;
                }

                trace!("Accepting new connection...");
                let s = Arc::clone(&s);
                task::spawn(async move { s.accept_connection(locked_stream(stream)).await });
            }

            info!("Terminating tcp accept loop!");
        });
    }

    /// loop over a stream of incoming data
    async fn accept_connection(self: Arc<Self>, stream: LockedStream) {
        let src_addr = match stream.write().await.as_mut().unwrap().peer_addr() {
            Ok(a) => a,
            Err(_) => {
                error!("Missing peer addr in stream; exiting!");
                return;
            }
        };

        loop {
            // Find the correct peer or create a temporary one.  If we
            // create a temporary one, we will need to upgrade it
            // before being able to accept valid connections.  We
            // update the peer on every iteration of the loop because
            // a previous packet might have upgraded the connection.
            let pid = self
                .routes
                .find_via_src(&src_addr)
                .await
                .unwrap_or_else(|| {
                    task::block_on(async { self.routes.add_via_src(&src_addr).await })
                });
            let peer = self.routes.get_peer(pid).await.unwrap();

            let f = {
                let mut stream = stream.write().await;

                let mut fb = PacketBuilder::new(stream.as_mut().unwrap());
                if let Err(_) = fb.parse().await {
                    error!("Failed to read from incoming packet stream; dropping connection!");
                    break;
                }

                match fb.build() {
                    Some(f) => f,
                    None => {
                        error!("Malformed frame; skipping!");
                        continue;
                    }
                }
            };

            // Match on the peer-state, message payload tuple.  Each
            // scenario is documented on the handler function to keep
            // this match block as small and readable as possible.
            // Avoid useless logging in this block too!
            use Packet::*;
            match (peer.state(), f) {
                (_, Frame(f)) => self.handle_frame(peer.id, f).await,
                (state, Hello { port, _type }) => {
                    self.handle_hello(peer.id, state, &src_addr, port, _type, Arc::clone(&stream))
                        .await
                }
                (_, Ack) => trace!("Received ACK packet on wrong i/o stream. woops"),
                #[allow(unreachable_patterns)]
                (state, packet) => panic!("state={:?}, packet={:?}", state, packet),
            }
        }

        self.routes.purge_src(src_addr).await;
        info!("Exiting connetion work-loop; was there a connection drop?");
    }

    /// Handle an incoming frame message
    async fn handle_frame(self: &Arc<Self>, peer_id: usize, p: Frame) {
        self.incoming.tx.send((p, peer_id)).await.unwrap();
    }

    /// Handle an incoming HELLO message on Tx, or Rx only connections
    ///
    /// A hello can come from a peer that we have said hello to before
    /// (TxOnly), or a peer that has just introduced itself without us
    /// knowing it before (RxOnly).  If the node is running in dynamic
    /// mode, check if the peer is in the set of "theoretically known
    /// peers" before accepting the hello.
    async fn handle_hello(
        self: &Arc<Self>,
        rx_peer: usize,
        state: PeerState,
        src: &SourceAddr,
        port: u16,
        _type: LinkType,
        stream: LockedStream,
    ) {
        let maybe_id = self.routes.find_via_srcport(src, port).await;
        let upm = "Received HELLO from unknown peer.";

        let s = match _type {
            // This connection needs to be established as the
            // reverse channel on this stream
            LinkType::Limited => {
                debug!("Receiving a limited incoming connection...");
                Some(Arc::clone(&stream))
            }
            // This is the default outgoing stream
            LinkType::Bidirect => None,
        };

        use PeerState::*;
        let _self = Arc::clone(self);
        match (state, self.mode, maybe_id) {
            // A peer we didn't know before, while running in dynamic mode
            (RxOnly, Mode::Dynamic, None) => {
                let id = self.routes.upgrade(rx_peer, port, s).await;
                trace!("Sending a hello...");
                self.send_hello(id, stream).await;
            }
            // Reverse connection of a peer we have known before
            (RxOnly, _, Some(_id)) => {
                let id = self.routes.upgrade(rx_peer, port, s).await;
                trace!("Sending a hello...");
                self.send_hello(id, stream).await;
            }
            (TxOnly, _, Some(id)) => {
                self.routes.add_src(id, *src).await;
                self.send_hello(id, stream).await;
            }
            // A peer we didn't know before, while running in static mode
            (_, Mode::Static, None) => {
                debug!("{} Running STATIC: dropping packet!", upm);
                return;
            }
            (link, mode, id) => panic!("{:?} {:?} {:?}", link, mode, id),
        }
    }

    async fn send_hello(self: &Arc<Self>, id: usize, stream: LockedStream) {
        let mut stream = stream.write().await;
        let buf = Packet::Ack.serialize();
        (*stream.as_mut().unwrap()).write_all(&buf).await.unwrap();

        let s = Arc::clone(self);
        task::spawn(async move {
            if let Some(peer) = s.routes.get_peer(id).await {
                task::sleep(Duration::from_secs(2)).await;
                peer.send(Packet::Hello {
                    port: s._port,
                    _type: peer.link_type(),
                })
                .await;
            }
        });
    }
}
