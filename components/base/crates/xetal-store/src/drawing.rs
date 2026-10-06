//! The disk, with pictures: what the command line installs. Files and
//! the keyboard behave as on the disk; each picture shown (`[]S_HOW`) is
//! written to the next numbered file, `DIR/STEM-1.svg`, `DIR/STEM-2.svg`,
//! ..., and reported by the host's callback. With a script of lines
//! (`--events FILE`), `[]R_EAD`, `[]K_EY` and `[]E_VENT` read those
//! instead of standard input, and the end of the script is the end of
//! input.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{Disk, Store};

/// The disk, writing pictures as numbered SVG files in one directory.
pub struct Drawing {
    dir: PathBuf,
    stem: String,
    shown: AtomicUsize,
    notify: fn(&Path),
    script: Option<Mutex<VecDeque<String>>>,
}

impl Drawing {
    /// Pictures go to `dir/stem-N.svg`; `notify` hears each path written.
    pub fn new(dir: impl Into<PathBuf>, stem: &str, notify: fn(&Path)) -> Drawing {
        Drawing {
            dir: dir.into(),
            stem: stem.into(),
            shown: AtomicUsize::new(0),
            notify,
            script: None,
        }
    }

    /// Lines typed come from `lines` (a scripted queue of events, RS1)
    /// instead of standard input; blank lines and `#` comments are
    /// skipped, and after the last line input has ended. A line keeps
    /// its trailing spaces: the space bar is `key` and two spaces.
    pub fn scripted(mut self, lines: &str) -> Drawing {
        let kept = lines
            .lines()
            .map(str::trim_start)
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
            .map(String::from)
            .collect();
        self.script = Some(Mutex::new(kept));
        self
    }
}

impl Store for Drawing {
    fn get(&self, path: &str) -> Result<String, String> {
        Disk.get(path)
    }

    fn put(&self, path: &str, text: &str) -> Result<(), String> {
        Disk.put(path, text)
    }

    fn line(&self) -> Result<String, String> {
        let Some(script) = &self.script else {
            return Disk.line();
        };
        let mut lines = script.lock().map_err(|e| e.to_string())?;
        lines.pop_front().ok_or_else(|| "no more input".into())
    }

    fn show(&self, svg: &str) -> Result<(), String> {
        let n = self.shown.fetch_add(1, Ordering::SeqCst) + 1;
        let path = self.dir.join(format!("{}-{n}.svg", self.stem));
        Disk.put(&path.display().to_string(), svg)?;
        (self.notify)(&path);
        Ok(())
    }
}
