//! API wrapper structures

pub mod contacts;
// pub mod files;
pub mod messages;
pub mod users;

#[cfg(feature = "chat")]
pub mod chat;

#[cfg(feature = "voice")]
pub mod voice;

mod envelope;
pub use envelope::{Envelope, Request, Response};

mod responder;
pub use responder::Responder;

mod streamer;
pub(crate) use streamer::Subscriber;
pub use streamer::{StreamResponder, Streamer, SubId};

use async_trait::async_trait;
use libqaul::Qaul;

/// Apply an RPC structure to a libqaul instance
///
/// This trait is used to attach a new function to the qaul state
/// holder, without having to rely on feature flags to libqaul.
#[async_trait]
pub trait QaulExt {
    async fn apply<Response, Request>(&self, r: Request) -> Response
    where
        Response: Send + Sync,
        Request: Send + Sync + QaulRpc<Response = Response>;
}

#[async_trait]
impl QaulExt for Qaul {
    async fn apply<R, T>(&self, r: T) -> R
    where
        R: Send + Sync,
        T: Send + Sync + QaulRpc<Response = R>,
    {
        r.apply(self).await
    }
}

/// The RPC trait that get's access to the libqaul state
#[async_trait]
pub trait QaulRpc {
    type Response;
    async fn apply(self, qaul: &Qaul) -> Self::Response;
}
