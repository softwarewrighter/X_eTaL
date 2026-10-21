//! A page's own libraries (ask D16): a page adds its `.xtl` or `.xtlm`
//! files to the store a run reads, so its programs import them by name,
//! and asks for a program after its macros expand, as `xetal expand`
//! prints it, to show what a macro wrote.

use xetal_base::Diagnostic;
use xetal_macro::StoreLibraries;
use xetal_notate::{drawn, found};
use xetal_program::{expanded_with, name_for};
use xetal_report::{Report, report_with};
use xetal_view::{Segment, view};

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

/// The lint report of `src` (the Lint button): its findings as
/// `xetal lint` prints them and the text with their fixes made, its
/// libraries found as a run finds them.
pub fn lint(src: &str) -> Report {
    report_with(name_for(src), src, &StoreLibraries)
}

/// `src` drawn decorated for the Rendered pane, its notation calls
/// (MC32) drawn as their `draws:` macros wrote them: one segment of
/// class `Notation` holding the LaTeX in place of the typed strings.
/// When `src` does not load, it is drawn as typed.
pub fn decorated(src: &str) -> Vec<Segment> {
    let notations = found(name_for(src), src, &StoreLibraries);
    drawn(view(src), &notations)
}
