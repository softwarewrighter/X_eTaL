//! A host without a disk (the browser) says so once; then the disk
//! finder, what a plain load uses, finds libraries in the store. Its own
//! test binary: the switch is process-wide.

use std::sync::Arc;

use xetal_lookup::{FsLibraries, Libraries, libraries_in_store};
use xetal_store::Store;

#[test]
fn a_host_without_a_disk_finds_libraries_in_its_store() {
    let store = xetal_store::Memory::default();
    store.put("Mine.xtl", "l:o_ne := { _u -> 1 }").unwrap();
    xetal_store::install(Arc::new(store));
    let libs = FsLibraries::new(Vec::new());
    assert!(libs.find("Mine", "-e").is_none(), "not on disk");
    libraries_in_store(true);
    let found = libs.find("Mine", "-e").map(|f| f.text);
    let standard = libs.find("Stats", "-e").is_some();
    libraries_in_store(false);
    assert_eq!(found.as_deref(), Some("l:o_ne := { _u -> 1 }"));
    assert!(standard, "the standard libraries still");
}
