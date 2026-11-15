//! Travel back in a run (a debugger's step back, Saga 43): a [`Mark`]
//! says how far the run had got with pictures, lines read and files
//! written; [`rewind`] goes back to one, so the pictures shown since are
//! not shown again, the lines read since are read again (as typed the
//! first time) and the files written since are not written again (they
//! stay as written: [`written`] names them).

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::pictures::{ASKED, NEXT, SKIP};

static WRITES: AtomicUsize = AtomicUsize::new(0);
static WRITES_SKIP: AtomicUsize = AtomicUsize::new(0);
/// The path of every file the run has written, in order.
static WRITTEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// How far a run had got.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Mark {
    pictures: usize,
    lines: usize,
    writes: usize,
}

/// Where the run is now.
pub fn mark() -> Mark {
    Mark {
        pictures: ASKED.load(Ordering::SeqCst),
        lines: NEXT.load(Ordering::SeqCst),
        writes: WRITES.load(Ordering::SeqCst),
    }
}

/// Go back to `to`: what the run did after it, it does not do again.
pub fn rewind(to: Mark) {
    let back = |count: &AtomicUsize, skip: &AtomicUsize, to: usize| {
        skip.fetch_max(count.load(Ordering::SeqCst), Ordering::SeqCst);
        count.store(to, Ordering::SeqCst);
    };
    back(&ASKED, &SKIP, to.pictures);
    back(&WRITES, &WRITES_SKIP, to.writes);
    NEXT.store(to.lines, Ordering::SeqCst);
}

/// The files written between `from` and `to`, in order.
pub fn written(from: Mark, to: Mark) -> Vec<String> {
    let all = WRITTEN.lock().unwrap_or_else(|e| e.into_inner());
    all.get(from.writes..to.writes.min(all.len()))
        .map(<[String]>::to_vec)
        .unwrap_or_default()
}

/// Count a file write to `path`; true when it should really be made
/// (a run gone back does not make again the writes it made before).
pub fn admit_write(path: &str) -> bool {
    let n = WRITES.fetch_add(1, Ordering::SeqCst);
    if n < WRITES_SKIP.load(Ordering::SeqCst) {
        return false;
    }
    let mut all = WRITTEN.lock().unwrap_or_else(|e| e.into_inner());
    all.truncate(n);
    all.push(path.to_string());
    true
}

/// A new run: no writes yet.
pub(crate) fn forget_writes() {
    WRITES.store(0, Ordering::SeqCst);
    WRITES_SKIP.store(0, Ordering::SeqCst);
    WRITTEN.lock().unwrap_or_else(|e| e.into_inner()).clear();
}
