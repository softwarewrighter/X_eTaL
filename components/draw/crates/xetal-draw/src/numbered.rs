//! Numbers drawn on a grid (QD8): each cell filled from a scale, its
//! number written in it, frame by frame.

use xetal_svg::anim::frames;
use xetal_svg::model::{DrawError, Layout};
use xetal_svg::palette::range;
use xetal_svg::scales::{ink_on, scaled};
use xetal_svg::svg::{cell, grid_document, label};

/// The numbers of this shape on a grid, filled from `scale` (0 gray,
/// 1 heat, 2 diverging); `texts` are the numbers as they are written.
pub fn numbered(
    shape: &[usize],
    v: &[f64],
    texts: &[String],
    scale: u32,
) -> Result<String, DrawError> {
    let layout = Layout::of(shape, v.len())?;
    let (lo, hi) = range(v);
    let n = layout.per_frame();
    let bodies: Vec<String> = (0..layout.frames)
        .map(|k| {
            (k * n..(k + 1) * n)
                .map(|i| {
                    let (r, c) = ((i - k * n) / layout.cols, (i - k * n) % layout.cols);
                    let [red, green, blue] = scaled(scale, v[i], lo, hi);
                    let fill = format!("#{red:02x}{green:02x}{blue:02x}");
                    let ink = ink_on([red, green, blue]);
                    cell(r, c, &fill) + &label(r, c, &texts[i], ink)
                })
                .collect()
        })
        .collect();
    Ok(grid_document(&layout, &frames(&bodies)))
}
