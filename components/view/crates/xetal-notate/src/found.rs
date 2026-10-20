//! The notations of a file, as a view takes them: the file is loaded as
//! `xetal run` loads it (its libraries from `libs`), which runs each
//! notation call's `draws:` macro and records the drawing on the call's
//! site; a file that does not load has no notations, and is drawn as
//! typed.

use xetal_macro::Libraries;
use xetal_program::load_with;
use xetal_sources::Sources;

use crate::Notation;

/// The notation calls in `src` (reported as `name`, which says where
/// its libraries are looked for), in order; none when it does not load.
pub fn found(name: &str, src: &str, libs: &dyn Libraries) -> Vec<Notation> {
    match load_with(name, src, libs) {
        Ok(loaded) => of(&loaded.sources),
        Err(_) => Vec::new(),
    }
}

/// The notation calls of a loaded program's own file.
pub fn of(sources: &Sources) -> Vec<Notation> {
    xetal_program::notations(0, sources)
        .into_iter()
        .map(|n| Notation {
            left: n.left,
            right: n.right,
            latex: n.latex,
        })
        .collect()
}
