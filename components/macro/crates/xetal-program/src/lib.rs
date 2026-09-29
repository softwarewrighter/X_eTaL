//! A program with its libraries, ready to check and run: the macro
//! phase (`xetal-macro`, libraries on disk and built in), then Core.
//! Every error comes back located where it was written, as a plain
//! diagnostic for one file or `at FILE:LINE:COLUMN` for several.

mod load;

pub use load::{Loaded, in_program, load, located, program_types};
