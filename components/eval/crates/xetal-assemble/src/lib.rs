//! Built-ins that assemble a new array from parts: `a_mend` (A8), cells
//! replaced at positions, and `m_ix` (A9), boxes made one array.

mod amend;
mod mix;

pub use amend::amend;
pub use mix::mix;
