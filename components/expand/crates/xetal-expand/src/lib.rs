//! Macro expansion (lang-choices MC12, MC17, MC18-MC22): each macro
//! call replaced by the text its macro gives (system macros and those
//! of macro libraries alike, run through a [`Macros`] table), again and
//! again to a depth limit, with a map from the expanded text back to
//! where each byte was written (text copied from a macro's argument
//! maps into the string it was written in; the macro's own text maps
//! to the whole call). Hygiene is by choice (MC30): names a macro
//! writes stay as written, a macro asks for fresh ones (`[]G_ENSYM`),
//! and the call's text cannot bind a macro's declared anaphora. A
//! macro's contract (MC31, `xetal-contract`) is checked around each
//! call: the shape it takes before it runs, and after, that a copying
//! macro used every name and number of its arguments. `u_se<` is left
//! for the imports.

mod calls;
mod copied;
mod expand;
mod hygiene;
mod user;

pub use calls::{Call, calls};
pub use expand::{DEPTH, expand, expand_with};
pub use user::{MacroCall, Macros, NoMacros};
pub use xetal_contract::{Contract, Uses, contracts_of};
pub use xetal_mapped::{Mapped, Piece};
