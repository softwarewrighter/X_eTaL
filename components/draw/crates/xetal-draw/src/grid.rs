//! A grid of cells, frame by frame.

use crate::anim::frames;
use crate::model::{Cells, DrawError, Layout};
use crate::palette::{INK, bits, color, range};
use crate::svg::{cell, document, glyph};

/// The array of this shape and these cells as one SVG document: a
/// scalar, vector or matrix as one grid, a rank-3 array as frames.
pub fn grid(shape: &[usize], cells: &Cells) -> Result<String, DrawError> {
    let layout = Layout::of(shape, cells.len())?;
    let bodies: Vec<String> = (0..layout.frames)
        .map(|k| frame(&layout, cells, k))
        .collect();
    Ok(document(&layout, &frames(&bodies)))
}

/// The cells of frame k.
fn frame(layout: &Layout, cells: &Cells, k: usize) -> String {
    let n = layout.per_frame();
    let at = |i: usize| (i / layout.cols, i % layout.cols);
    match cells {
        Cells::Chars(v) => (0..n)
            .filter(|&i| v[k * n + i] != ' ')
            .map(|i| glyph(at(i).0, at(i).1, v[k * n + i]))
            .collect(),
        Cells::Numbers(v) => numbers(v, k * n..(k + 1) * n, &at),
    }
}

/// Numbers: dark cells for the 1s of a bit array, else every cell colored.
fn numbers(
    v: &[f64],
    span: std::ops::Range<usize>,
    at: &dyn Fn(usize) -> (usize, usize),
) -> String {
    let start = span.start;
    if bits(v) {
        return span
            .filter(|&i| v[i] == 1.0)
            .map(|i| cell_at(at(i - start), INK))
            .collect();
    }
    let (lo, hi) = range(v);
    span.map(|i| cell_at(at(i - start), &color(v[i], lo, hi)))
        .collect()
}

fn cell_at((r, c): (usize, usize), fill: &str) -> String {
    cell(r, c, fill)
}
