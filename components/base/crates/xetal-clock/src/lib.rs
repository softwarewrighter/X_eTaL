//! The clock a program reads (`[]TS`, local time) and waits on
//! (`[]D_L`), QD2, QD3, QD7: the system's on the command line, or one
//! a host installs (the browser's worker installs the page's). A host
//! without a clock answers with an error, never a panic.

mod clock;
mod current;

pub use clock::{Clock, Stamp, System};
pub use current::{delay, install, now};
