use crate::{
    config::{Config, InOrOut},
    inlet::Inlet,
    outlet::Outlet,
};
use async_std::{
    net::TcpStream,
    sync::{Arc, RwLock},
};
use ratman_client::Identity;
use std::collections::BTreeMap;

pub type SessionMap = Arc<RwLock<BTreeMap<Identity, TcpStream>>>;

/// The main proxy server state
pub struct Server {
    #[allow(unused)]
    cfg: Config,
    map: SessionMap,
}

impl Server {
    pub fn new(cfg: Config) -> Self {
        Self {
            cfg,
            map: SessionMap::default(),
        }
    }

    /// Run this server
    pub async fn run(&self, bind: Option<&str>) {
        for (ip, (io, addr)) in self.cfg.map.iter() {
            if let Err(e) = match io {
                InOrOut::In => Inlet::new(bind, ip, *addr),
                InOrOut::Out => Outlet::new(&self.map, bind, ip, *addr),
            } {
                error!(
                    "failed to initialise {}: {}",
                    match io {
                        InOrOut::In => "inward socket",
                        InOrOut::Out => "outward socket",
                    },
                    e
                );
            }
        }

        // wowow this is a hack ;_;
        async_std::future::pending().await
    }
}
