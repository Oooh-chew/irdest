//! **A clock control mechanism for internally scheduled task
//! runners**
//!
//! This library was primarily written as a utility for [Ratman], and
//! libqaul, but can be used in any reactor setting where direct
//! scheduling control should be possible without having to expose all
//! tasks from it.
//!
//! [Ratman]: https://crates.io/crate/ratman
//!
//! ## Example: Ratman
//!
//! By default, each detached task inside Ratman is run at the speed
//! that the hardware allows, i.e. polling tasks will not wait between
//! poll loops.  This is usually fine, on systems that are not battery
//! or CPU constrained.  However, on systems that are, it can cause
//! huge battery drain.  This is where [`ClockCtrl`] comes in, a clock
//! receiver which can be configured with various types to manipulate
//! the runtime behaviour of the internal tasks running inside Ratman.
//!
//! [`ClockCtrl`]: struct.ClockCtrl.html
//!
//! ```
//! use clockctrl::{ClockCtrl, Error, Interval, Scheduler};
//! use std::time::Duration;
//!
//! # #[derive(Hash, Eq, PartialEq, Ord, PartialOrd)] enum MyTasks { Journal, Switch }
//! let mut clc = ClockCtrl::new();
//! clc.setup(MyTasks::Journal)
//!     .set(Interval::Timed(Duration::from_secs(10)));
//!
//! clc.setup(MyTasks::Switch)
//!     .set(Interval::Stepped)
//!     .fence(move |_| {
//!         // ...
//!     });
//! ```

#![doc(html_favicon_url = "https://qaul.org/favicon.ico")]
#![doc(html_logo_url = "https://qaul.org/img/qaul_icon-128.png")]

mod ctrl;
pub use ctrl::{ClockCtrl, Scheduler};

mod error;
pub use error::Error;

mod target;
pub use target::{Interval, Target};
