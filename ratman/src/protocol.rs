// SPDX-FileCopyrightText: 2019-2022 Katharina Fey <kookie@spacekookie.de>
//
// SPDX-License-Identifier: AGPL-3.0-or-later WITH LicenseRef-AppStore

//! Protocol generation module
//!
//! The routing protocol, and micro messages (analogous to micro
//! code), are much better documented in the `R.A.T.M.A.N.` design
//! specification/paper. But here's a brief overview, and
//! implementation:
//!
//! - `Announce` is sent when a node comes online
//! - `Sync` is a reply to an `Announce`, only omitted when `no_sync` is set

use crate::Core;
use async_std::{
    sync::{Arc, Mutex},
    task,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use types::{Error, Frame, Identity, Result};

/// A payload that represents a RATMAN-protocol message
#[derive(Debug, Serialize, Deserialize)]
enum ProtoPayload {
    /// A network-wide announcement message
    Announce { id: Identity, no_sync: bool },
}

/// Provide a builder API to construct different types of Messages
#[derive(Default)]
pub(crate) struct Protocol {
    online: Mutex<BTreeMap<Identity, Arc<AtomicBool>>>,
}

impl Protocol {
    pub(crate) fn new() -> Arc<Self> {
        Default::default()
    }

    /// Dispatch a task to announce a user periodically
    pub(crate) async fn online(self: Arc<Self>, id: Identity, core: Arc<Core>) -> Result<()> {
        let mut map = self.online.lock().await;
        if map.get(&id).map(|arc| arc.load(Ordering::Relaxed)) == Some(true) {
            // If a user is already online we don't have to do anything
            return Ok(());
        }

        info!("Setting address {} to 'online'", id);

        let b = Arc::new(AtomicBool::new(true));
        map.insert(id, Arc::clone(&b));
        drop(map);

        task::spawn(async move {
            loop {
                debug!("Sending announcement for {}", id);
                core.raw_flood(Self::announce(id)).await.unwrap();
                task::sleep(Duration::from_secs(2)).await;

                if !b.load(Ordering::Relaxed) && break {}
            }

            // Remove the runtime bool again
            self.online.lock().await.remove(&id);
        });

        Ok(())
    }

    pub(crate) async fn offline(&self, id: Identity) -> Result<()> {
        info!("Setting address {} to 'offline'", id);
        self.online
            .lock()
            .await
            .get(&id)
            .map(|b| b.swap(false, Ordering::Relaxed))
            .map_or(Err(Error::NoAddress), |_| Ok(()))
    }

    /// Try to parse a frame as an announcement
    pub(crate) fn is_announce(f: &Frame) -> Option<Identity> {
        let Frame { ref payload, .. } = f;

        bincode::deserialize(payload)
            .map(|p| match p {
                ProtoPayload::Announce { id, .. } => id,
            })
            .ok()
    }

    /// Build an announcement message for a user
    fn announce(sender: Identity) -> Frame {
        let payload = bincode::serialize(&ProtoPayload::Announce {
            id: sender,
            no_sync: true,
        })
        .unwrap();

        // Currently we just use the sender address as the "scope" of the
        Frame::inline_flood(sender, sender, payload)
    }
}
