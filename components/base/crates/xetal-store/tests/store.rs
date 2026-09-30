//! The file store: the disk by default, another one when installed.

use std::sync::Arc;

use xetal_store::{Memory, Store, install, read, write};

#[test]
fn a_memory_store_keeps_what_is_written() {
    let store = Memory::default();
    store.put("work/m.txt", "1 2 3").unwrap();
    assert_eq!(store.get("work/m.txt").unwrap(), "1 2 3");
    assert!(store.get("nothing.txt").is_err());
    assert_eq!(store.paths(), ["work/m.txt"]);
}

#[test]
fn the_installed_store_serves_reads_and_writes() {
    let store = Arc::new(Memory::default());
    install(store.clone());
    write("work/a.txt", "hello").unwrap();
    assert_eq!(read("work/a.txt").unwrap(), "hello");
    assert_eq!(store.get("work/a.txt").unwrap(), "hello");
    assert!(read("missing.txt").unwrap_err().contains("missing.txt"));
}
