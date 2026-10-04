//! Macro expansion (lang-choices MC12, MC14-MC17): the system macros
//! `i_f<`, `u_nless<` and `e_ach<` replaced by the source they stand
//! for, again and again to a depth limit, with a map from the expanded
//! text back to where each byte was written (code from a macro's
//! argument maps into the string it was written in; the macro's own
//! text maps to the whole call). `u_se<` is left for the imports.

mod calls;
mod each;
mod expand;
mod system;
mod user;

pub use each::WORD;
pub use expand::{DEPTH, expand, expand_with};
pub use system::SYSTEM;
pub use user::{Macros, NoMacros};
pub use xetal_mapped::{Mapped, Piece};
