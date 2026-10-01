//! Files kept in memory, and lines queued as if typed (for tests, and a
//! host without a file system).

use std::collections::{BTreeMap, VecDeque};
use std::sync::Mutex;

use crate::Store;

/// Files kept in memory.
#[derive(Default)]
pub struct Memory {
    files: Mutex<BTreeMap<String, String>>,
    typed: Mutex<VecDeque<String>>,
    pictures: Mutex<Vec<String>>,
}

impl Memory {
    /// The stored paths, in order.
    pub fn paths(&self) -> Vec<String> {
        self.files
            .lock()
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// The pictures shown, in order.
    pub fn pictures(&self) -> Vec<String> {
        self.pictures.lock().map(|p| p.clone()).unwrap_or_default()
    }

    /// Queue `line` as if typed at the keyboard.
    pub fn push_line(&self, line: &str) {
        if let Ok(mut typed) = self.typed.lock() {
            typed.push_back(line.into());
        }
    }
}

impl Store for Memory {
    fn get(&self, path: &str) -> Result<String, String> {
        let files = self
            .files
            .lock()
            .map_err(|_| "the store is unusable".to_string())?;
        files
            .get(path)
            .cloned()
            .ok_or_else(|| format!("{path}: no such file"))
    }

    fn put(&self, path: &str, text: &str) -> Result<(), String> {
        let mut files = self
            .files
            .lock()
            .map_err(|_| "the store is unusable".to_string())?;
        files.insert(path.into(), text.into());
        Ok(())
    }

    fn line(&self) -> Result<String, String> {
        let mut typed = self
            .typed
            .lock()
            .map_err(|_| "the store is unusable".to_string())?;
        typed.pop_front().ok_or_else(|| "no more input".to_string())
    }

    fn show(&self, svg: &str) -> Result<(), String> {
        let mut pictures = self
            .pictures
            .lock()
            .map_err(|_| "the store is unusable".to_string())?;
        pictures.push(svg.into());
        Ok(())
    }
}
