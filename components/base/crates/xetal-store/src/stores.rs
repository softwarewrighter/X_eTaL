//! Stores: the disk, and memory (for tests, and a host without one).

use std::collections::BTreeMap;
use std::sync::Mutex;

/// A place files are read from and written to, by path.
pub trait Store: Send + Sync {
    fn get(&self, path: &str) -> Result<String, String>;
    fn put(&self, path: &str, text: &str) -> Result<(), String>;
}

/// The file system; writing makes missing directories.
pub struct Disk;

impl Store for Disk {
    fn get(&self, path: &str) -> Result<String, String> {
        std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))
    }

    fn put(&self, path: &str, text: &str) -> Result<(), String> {
        let dir = std::path::Path::new(path).parent();
        if let Some(dir) = dir.filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir).map_err(|e| format!("{path}: {e}"))?;
        }
        std::fs::write(path, text).map_err(|e| format!("{path}: {e}"))
    }
}

/// Files kept in memory.
#[derive(Default)]
pub struct Memory(Mutex<BTreeMap<String, String>>);

impl Memory {
    /// The stored paths, in order.
    pub fn paths(&self) -> Vec<String> {
        self.0
            .lock()
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl Store for Memory {
    fn get(&self, path: &str) -> Result<String, String> {
        let files = self
            .0
            .lock()
            .map_err(|_| "the store is unusable".to_string())?;
        files
            .get(path)
            .cloned()
            .ok_or_else(|| format!("{path}: no such file"))
    }

    fn put(&self, path: &str, text: &str) -> Result<(), String> {
        let mut files = self
            .0
            .lock()
            .map_err(|_| "the store is unusable".to_string())?;
        files.insert(path.into(), text.into());
        Ok(())
    }
}
