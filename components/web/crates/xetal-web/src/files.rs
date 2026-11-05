//! The files the terminal page's shell sees: your files in the store
//! first, then the demos by their names (`life.xtl`,
//! `classics/hanoi.xtl`), then the standard libraries (`Stats.xtl`,
//! `Macros.xtlm`), then X_eTaL-libraries' collection (`Equations.xtl`,
//! its demos as `Equations/integrals.xtl`).

use xetal_webtui::Files;

use crate::{DEMOS, storage};

/// The shell's files.
pub(crate) const FILES: Files = Files { read, list };

/// The text of `path`: a saved file, a demo, or a standard library.
fn read(path: &str) -> Option<String> {
    if let Ok(text) = xetal_store::read(path) {
        return Some(text);
    }
    if let Some(demo) = DEMOS.iter().find(|d| d.name == path) {
        return Some(demo.text.to_string());
    }
    let library = |ext: &str| path.strip_suffix(ext);
    let standard = match (library(".xtl"), library(".xtlm")) {
        (Some(n), _) => xetal_libs::standard(n),
        (_, Some(n)) => xetal_libs::standard_macros(n),
        _ => None,
    };
    let collected = || xetal_collection::library(path).or_else(|| xetal_collection::demo(path));
    standard.or_else(collected).map(String::from)
}

/// What `ls` lists: your files, the demos, the standard libraries.
fn list() -> Vec<String> {
    let demos = DEMOS
        .iter()
        .map(|d| d.name.to_string())
        .filter(|n| n != "(empty)");
    let libs = xetal_libs::LIBRARIES
        .iter()
        .map(|(n, _)| format!("{n}.xtl"));
    let macros = xetal_libs::MACROS.iter().map(|(n, _)| format!("{n}.xtlm"));
    let collection = xetal_collection::LIBRARIES
        .iter()
        .chain(xetal_collection::DEMOS);
    storage::saved()
        .into_iter()
        .chain(demos)
        .chain(libs)
        .chain(macros)
        .chain(collection.map(|(n, _)| n.to_string()))
        .collect()
}
