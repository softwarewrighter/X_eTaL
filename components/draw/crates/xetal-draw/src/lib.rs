//! Arrays drawn as self-contained SVG (lang-choices QD5): a matrix as a
//! grid of cells, a 2-row matrix of points as a path, a rank-3 array
//! as frames shown in turn. Pure text in,
//! text out: it knows nothing of the language, only shapes and cells,
//! so the command line, the browser and any other host show the same
//! picture.

mod anim;
mod grid;
mod model;
mod palette;
mod path;
mod svg;

pub use grid::grid;
pub use model::{Cells, DrawError};
pub use path::path;
