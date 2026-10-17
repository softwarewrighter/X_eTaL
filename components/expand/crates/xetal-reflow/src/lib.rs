//! Laying an expansion out as a program (D159): text a macro produced
//! is often one long line. Where a line is longer than [`WIDTH`], a
//! line break is put where one is free: inside a brace block one
//! statement per line (a `;` becomes the line's end), nested blocks
//! indented, and a long expression broken at a space inside its
//! parentheses. Tokens keep their spelling, their order and their map
//! back to where they were written; nothing is broken where no space
//! was (`x -1`, `x^2`), inside brackets, or in a string.

mod blocks;
mod emit;
mod lay;

pub use lay::{WIDTH, laid_out};
