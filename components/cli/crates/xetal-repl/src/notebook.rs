//! A file run as a notebook: each statement (one line, or several while
//! a bracket is open) with the output and errors it produced, from a
//! session fed line by line (so definitions persist, printing is not
//! repeated and one seed keeps the rolls consistent).

use crate::{Reply, Session};

/// One statement's source and what running it printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub source: String,
    pub out: String,
    pub err: String,
}

/// The cells of `src`, rolls drawn from `seed`.
pub fn notebook(src: &str, seed: u64) -> Vec<Cell> {
    let mut session = Session::seeded(seed);
    let (mut cells, mut source) = (Vec::new(), Vec::new());
    for line in src.lines() {
        source.push(line);
        if let Reply::Done { out, err } = session.feed(line) {
            cells.push(Cell {
                source: source.join("\n"),
                out,
                err,
            });
            source.clear();
        }
    }
    if !source.is_empty() {
        let err = "error[unclosed]: the file ends inside a bracket\n".to_string();
        cells.push(Cell {
            source: source.join("\n"),
            out: String::new(),
            err,
        });
    }
    cells
}
