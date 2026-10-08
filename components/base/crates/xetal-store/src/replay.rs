//! Pictures, lines read and replays. A session runs its accepted
//! source again for each new line, so the pictures that source shows
//! were shown before, and the lines it read were read before:
//! `replay(n)` at the start of a run lets the first n pictures through
//! silently and serves the lines read so far again, in order, before
//! reading new ones; `shown()` after it says how many pictures the run
//! asked for. `muted(true)` counts pictures without showing them (a
//! context run silently). The counts are process-wide, as the store is.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static SKIP: AtomicUsize = AtomicUsize::new(0);
static ASKED: AtomicUsize = AtomicUsize::new(0);
static MUTED: AtomicBool = AtomicBool::new(false);
/// Every line read in this process, and how many this run has read.
static READ: Mutex<Vec<String>> = Mutex::new(Vec::new());
static NEXT: AtomicUsize = AtomicUsize::new(0);

/// Start a run whose first `skip` pictures were shown before.
pub fn replay(skip: usize) {
    SKIP.store(skip, Ordering::SeqCst);
    ASKED.store(0, Ordering::SeqCst);
    NEXT.store(0, Ordering::SeqCst);
}

/// How many pictures this run has asked to show (skipped ones too).
pub fn shown() -> usize {
    ASKED.load(Ordering::SeqCst)
}

/// While muted, pictures are counted but not shown.
pub fn muted(on: bool) {
    MUTED.store(on, Ordering::SeqCst);
}

/// Count a picture; true when it should really be shown.
pub(crate) fn admit() -> bool {
    let n = ASKED.fetch_add(1, Ordering::SeqCst);
    n >= SKIP.load(Ordering::SeqCst) && !MUTED.load(Ordering::SeqCst)
}

/// Forget the lines read so far: a new session reads afresh.
pub fn forget_lines() {
    READ.lock().unwrap_or_else(|e| e.into_inner()).clear();
    NEXT.store(0, Ordering::SeqCst);
}

/// The next line for `[]R_EAD`: one an earlier run of the same source
/// read, else a new one from `read`, kept for the runs after.
pub(crate) fn line(read: impl FnOnce() -> Result<String, String>) -> Result<String, String> {
    let n = NEXT.fetch_add(1, Ordering::SeqCst);
    let mut lines = READ.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(seen) = lines.get(n) {
        return Ok(seen.clone());
    }
    let new = read().inspect_err(|_| {
        NEXT.fetch_sub(1, Ordering::SeqCst);
    })?;
    lines.push(new.clone());
    Ok(new)
}
