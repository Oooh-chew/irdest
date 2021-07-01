//! I/O persistence module

pub(crate) mod format;

mod sync;
pub use sync::Sync;

mod cfg;
pub use cfg::legacy as versions;
pub use cfg::Config;