use super::QaulRpc;
use crate::Response;
use async_std::{
    sync::{Arc, RwLock},
    task,
};
use async_trait::async_trait;
use libqaul::{error::Result, users::UserAuth, Identity, Qaul};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::BTreeSet, marker::PhantomData};

type Atomic = Arc<AtomicBool>;

/// A unique subscription Id
pub type SubId = Identity;

/// Cancel any subscription via it's ID
///
/// The implementation of QaulRPC handles verifying the token first.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CancelSub {
    pub auth: UserAuth,
    pub id: SubId,
}

#[async_trait]
impl QaulRpc for CancelSub {
    type Response = Result<SubId>;
    async fn apply(self, qaul: &Qaul) -> Self::Response {
        let id = self.id;
        qaul.users().is_authenticated(self.auth).await.map(|_| id)
    }
}

/// An RPC message streaming tracker
///
/// Because the RPC crate handles subscription states, without knowing
/// how to respond to a subscription, this type maps the polled
/// subscription object in a task, than then notifies a responder,
/// that is passed in by a more specific RPC layer (such as `http` or
/// `ws`, ...).  This way the libqaul-rpc crate doesn't have to care
/// about how to send something, and higher layers don't have to care
/// about duplicating stream tracking logic.
pub struct Streamer<T>
where
    T: StreamResponder + Send + Sync + 'static,
{
    resp: Arc<T>,
    map: RwLock<BTreeSet<SubId>>,
}

impl<T> Streamer<T>
where
    T: StreamResponder + Send + Sync + 'static,
{
    pub fn new(resp: T) -> Arc<Self> {
        Arc::new(Self {
            resp: Arc::new(resp),
            map: Default::default(),
        })
    }

    /// Start a new subscription and return the ID
    pub(crate) fn start<S>(self: &Arc<Self>, sub: S) -> SubId
    where
        S: Subscriber + Send + Sync + 'static,
    {
        let subid = SubId::random();
        let this = Arc::clone(self);

        task::spawn(async move {
            this.map.write().await.insert(subid);

            while let Some(t) = sub.next().await {
                // Kill the task if the subscription has died
                if !this.map.read().await.contains(&subid) && break {}

                let resp = t.into();
                Arc::clone(&this.resp).respond(resp).await;
            }
        });

        subid
    }

    /// Stop a subscription from streaming data past it's current operation
    pub(crate) async fn stop(self: &Arc<Self>, id: SubId) {
        self.map.write().await.remove(&id);
    }
}

/// A simple wrapper around a common subscription in qaul
#[async_trait]
pub(crate) trait Subscriber {
    async fn next(&self) -> Option<Response>;
}

/// Take a `Response` type and map it to an RPC responder type
#[async_trait]
pub trait StreamResponder {
    async fn respond(self: Arc<Self>, r: Response);
}
