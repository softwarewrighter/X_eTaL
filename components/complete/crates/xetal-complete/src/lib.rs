//! Completion for the editors (the live demo, `xetal edit`, the REPL):
//! the candidates for what is being typed at the cursor. A built-in or
//! a name the text defines (`r_` or `ran` offers `r_ange`, with its
//! type), a library name inside the string of `u_se<` (`"S` offers
//! `Stats`), an imported library's exports after its alias (`x:`), the
//! lambda arguments after `_` in a block, the levels and lint names
//! after `# lint:`. Pure: the host supplies the saved files and how a
//! library's text is found, and draws the list itself. The results are
//! sorted by label, so the first is what Tab accepts.

mod directive;
mod library;
mod names;
mod word;

pub use word::{Candidate, Completion, Context, complete};
