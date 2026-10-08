//! A page's own libraries (ask D16): a page adds its `.xtl` or `.xtlm`
//! files to the store a run reads, so its programs import them by name,
//! and asks for a program after its macros expand, as `xetal expand`
//! prints it, to show what a macro wrote.

use xetal_base::Diagnostic;
use xetal_macro::StoreLibraries;
use xetal_program::{expanded_with, name_for};

/// Add the library file `name` (`Name.xtl` or `Name.xtlm`) with `text`
/// to the store in use; a program imports it as `"x:" u_se< "Name"`.
pub fn add_library(name: &str, text: &str) -> Result<(), String> {
    xetal_store::write(name, text)
}

/// `src` after its macros expand, its libraries found as a run finds
/// them (the store in use, then the standard libraries).
pub fn expanded(src: &str) -> Result<String, Diagnostic> {
    expanded_with(name_for(src), src, &StoreLibraries)
}
