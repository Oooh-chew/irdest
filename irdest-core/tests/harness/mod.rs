//! A libqaul specific harness for arbitrary API types

use irdest_core::Irdest;
use ratman_harness::{temp, Initialize, ThreePoint};
use std::{sync::Arc, time::Duration};

pub use async_std::future::timeout;
pub use ratman_harness::{millis, sec10, sec5};

pub async fn zzz(dur: Duration) {
    async_std::task::sleep(dur).await
}

pub async fn init() -> ThreePoint<Arc<Irdest>> {
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| Irdest::new(arc));
    tp
}
