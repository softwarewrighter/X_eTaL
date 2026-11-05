//! Libraries from the store the host installed (`xetal-store`), then
//! the standard libraries built in: the live demo's libraries, where
//! the files are the browser's local storage. A name is `Name.xtl` and
//! `Name.xtlm` in the store (MC11); a path is looked up as written.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::disk::standard;
use crate::found::{Pair, Spec, spec};
use crate::{Found, Libraries};

pub struct StoreLibraries;

/// Whether the host has no disk of its own: then the disk finder
/// (`FsLibraries`, what a plain load uses) finds in the store instead.
static NO_DISK: AtomicBool = AtomicBool::new(false);

/// Say once, at start, that this host keeps its files in the store
/// (the browser): every load finds libraries as the live demo does.
pub fn libraries_in_store(on: bool) {
    NO_DISK.store(on, Ordering::SeqCst);
}

pub(crate) fn in_store() -> bool {
    NO_DISK.load(Ordering::SeqCst)
}

impl Libraries for StoreLibraries {
    fn find(&self, spec: &str, from: &str) -> Option<Found> {
        self.find_both(spec, from).0
    }

    fn find_both(&self, name: &str, _from: &str) -> Pair {
        match spec(name) {
            Spec::Library => (stored(name), None),
            Spec::Macros => (None, stored(name)),
            Spec::Name => match (
                stored(&format!("{name}.xtl")),
                stored(&format!("{name}.xtlm")),
            ) {
                (None, None) => match standard(name) {
                    (None, None) => (
                        collected(&format!("{name}.xtl")),
                        collected(&format!("{name}.xtlm")),
                    ),
                    pair => pair,
                },
                pair => pair,
            },
        }
    }
}

fn stored(path: &str) -> Option<Found> {
    let found = xetal_store::read(path).ok().map(|text| Found {
        key: format!("store:{path}"),
        name: path.into(),
        text,
    });
    found.or_else(|| collected(path))
}

/// The last place a library is found: a collection the host embeds
/// (the live demo's X_eTaL-libraries), by file name.
static COLLECTION: OnceLock<fn(&str) -> Option<&'static str>> = OnceLock::new();

/// Add a collection of library files, found by file name
/// (`Equations.xtl`) after the store and the standard libraries.
pub fn add_collection(find: fn(&str) -> Option<&'static str>) {
    let _ = COLLECTION.set(find);
}

fn collected(file: &str) -> Option<Found> {
    let text = (COLLECTION.get()?)(file)?;
    Some(Found {
        key: format!("collection:{file}"),
        name: file.into(),
        text: text.into(),
    })
}
