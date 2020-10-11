//! An extensible rpc message broker for the libqaul ecosystem.

use async_std::sync::RwLock;
use qrpc_sdk::{
    builders, default_socket_path,
    io::MsgReader,
    rpc::capabilities::{self, Which},
    PosixAddr, PosixSocket, RpcSocket,
};
use std::{collections::BTreeMap, sync::Arc};
use tracing::{error, info};

type CapReader = MsgReader<'static, capabilities::Reader<'static>>;

/// Hold the main broker state
pub struct Broker {
    sock: Arc<RpcSocket>,
    connections: Arc<RwLock<BTreeMap<String, Arc<PosixSocket>>>>,
}

impl Broker {
    pub fn new() -> Self {
        let sock = RpcSocket::create(default_socket_path()).unwrap();
        sock.start_server(|socket, addr| {
            Self::handle_connection(socket, addr);
        });

        let connections = Default::default();
        Self { sock, connections }
    }

    /// Handle connections from a single incoming socket
    fn handle_connection(sock: Arc<RpcSocket>, src_addr: PosixAddr) {
        info!("Receiving connection to: {:?}", src_addr);
        loop {
            let (_dst_addr, buffer) = match sock.recv() {
                Some(a) => a,
                None => break,
            };

            let capr: CapReader = match MsgReader::new(buffer) {
                Ok(r) => r,
                Err(_) => {
                    sock.send_raw(builders::resp_err(), None);
                    continue;
                }
            };

            match capr.get_root().unwrap().which() {
                Ok(Which::Register(Ok(reg))) => {}
                Ok(Which::Unregister(Ok(unreg))) => {}
                Ok(Which::Upgrade(Ok(upgr))) => {}
                _ => {
                    error!("Invalid capability set; dropping connection");
                    continue;
                }
            }
        }
    }
}

// #[test]
// fn make_it_just_work_please() {
//     use capnp::{message::Builder, serialize_packed};
//     use qrpc_sdk::types::rpc_broker::service;

//     let mut msg = Builder::new_default();
//     let mut service = msg.init_root::<service::Builder>();

//     let d = "This is a test service to see how the RPC layer works";

//     service.set_name("net.qaul.test-service");
//     service.set_version(1);
//     service.set_description(d.clone());

//     let mut buffer = vec![];
//     serialize_packed::write_message(&mut buffer, &msg).unwrap();

//     //// Now test our de-serialisation logic
//     let reader = UtilReader::new(buffer).unwrap();
//     let parsed: service::Reader = reader.get_root().unwrap();

//     assert_eq!(parsed.get_name().unwrap(), "net.qaul.test-service");
//     assert_eq!(parsed.get_description().unwrap(), d);
//     assert_eq!(parsed.get_version(), 1);
// }

/*
Stuff I need

service -> service
service -> libqaul
libqaul -> service (reply, push/ subscription)


Each service has two parts: service core, and service client lib

service client lib:

- no logic
- defines the API and types with capn proto

service core:

- all the logic
- no types
- connects to the broker to advertise it's capabilities

Service advertisement

- name
- hash id
- capabilities

*/