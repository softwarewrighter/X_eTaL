//! Time travel for the debugger (Saga 43, docs/debugger-plan.md section
//! 3): a run is deterministic once its seed and inputs are fixed, so
//! going back to step k is restoring the nearest checkpoint at or before
//! k and running forward to k. A [`Timeline`] keeps the checkpoints
//! (copies of the machine's state, with each interval's slot writes to
//! undo), thinned as the run grows so memory stays bounded; a [`Gate`]
//! lets the output through once, however often a stretch is run again.

mod gate;
mod timeline;

pub use gate::{Gate, Tally, gate};
pub use timeline::Timeline;
