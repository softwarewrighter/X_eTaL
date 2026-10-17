//! Lints over macro libraries (`.xtlm`, MC10; docs/type-check-macros.md
//! approach 7): what a macro author can get wrong that no call shows
//! until a user meets it. A fresh name asked for and never used, or
//! asked for under a name that is not plain (`[]G_ENSYM`); a code a
//! macro rejects its call with that its doc block never mentions, or
//! placed nowhere (`[]R_EJECT`); a macro with no `## >>` example. The
//! passes read Core and the file's `##` blocks; their catalog entries
//! are in xetal-lint with the rest.

mod example;
mod fresh;
mod macros;
mod reject;

pub use macros::merged;
