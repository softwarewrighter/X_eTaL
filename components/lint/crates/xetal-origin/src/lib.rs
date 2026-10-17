//! Lints that know where each piece of Core came from (docs/lint.md
//! section 8). Macro expansion keeps a map from every expanded byte to
//! where it was written: copied from a call's argument (the caller's
//! code) or written by the macro (mapped to the whole call). These
//! passes read that map, and the calls recorded with it (`Site`: the
//! macro as written and its `## binds:`):
//!
//! - `macro-capture`: a binding the macro wrote, not a gensym and not
//!   declared, read by the call's text: the caller's name was captured.
//! - [`placed`]: a finding in code the macro wrote is not the caller's
//!   to fix, so it is reported as `macro-internals` (off by default), or
//!   for an unread declared anaphor as `unused-anaphor`.

mod capture;
mod report;
mod site;

pub use capture::captures;
pub use report::placed;
