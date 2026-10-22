//! Tab completes the word at the cursor: the first candidate is written
//! over it, and Tab again cycles through the others; any other key ends
//! the cycle. Libraries are the standard ones.

use xetal_complete::{Context, complete};

use crate::Line;

impl Line {
    /// Tab: complete, or cycle to the next candidate.
    pub(crate) fn complete(&mut self) {
        let (completion, next) = match self.cycle.take() {
            Some((c, i)) => {
                let shown = c.candidates[i].insert.chars().count();
                for _ in 0..shown {
                    self.buffer.backspace();
                }
                let next = (i + 1) % c.candidates.len();
                (c, next)
            }
            None => {
                let library = |name: &str| xetal_libs::standard(name).map(String::from);
                let ctx = Context {
                    files: &[],
                    library: &library,
                };
                let Some(c) = complete(&self.buffer.text(), self.buffer.offset(), &ctx) else {
                    return;
                };
                let typed = self.buffer.text()[c.start..c.end].chars().count();
                for _ in 0..typed {
                    self.buffer.backspace();
                }
                (c, 0)
            }
        };
        for ch in completion.candidates[next].insert.chars() {
            self.buffer.insert(ch);
        }
        self.cycle = Some((completion, next));
    }
}
