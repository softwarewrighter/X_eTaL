//! Signature lines in the system macro library (T4, MC21): the
//! declarations of built-in system macros, read and blanked before the
//! file is lexed, and given to the doc model.

mod sigs;

pub use sigs::{Signature, signatures, system_signatures};
