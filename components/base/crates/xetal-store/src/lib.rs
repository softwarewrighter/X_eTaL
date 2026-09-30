//! Where a program's files live. The command line reads and writes the
//! disk; a host can install another store instead (the live demo
//! installs the browser's local storage), and both the system
//! built-ins (`[]N_GET`, `[]N_PUT`) and library lookup go through it.

mod current;
mod memory;
mod stores;

pub use current::{install, read, read_line, write};
pub use memory::Memory;
pub use stores::{Disk, Store};
