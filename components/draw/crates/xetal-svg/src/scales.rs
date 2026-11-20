//! The color scales of `[]N_GRID` (QD8): gray (light to dark), heat
//! (the grid's viridis, dark purple to yellow) and diverging (blue
//! through white to red, centered on 0), and an ink that reads on each.

use crate::palette::rgb;

/// Gray, the paper's light to the ink's dark.
const GRAY: [(u8, u8, u8); 2] = [(0xf8, 0xfa, 0xfc), (0x1f, 0x29, 0x37)];
/// Blue, white, red (ColorBrewer's RdBu ends).
const DIVERGING: [(u8, u8, u8); 3] = [(0x21, 0x66, 0xac), (0xf7, 0xf7, 0xf7), (0xb2, 0x18, 0x2b)];

/// A scale, by its index in `Scale` (0 gray, 1 heat, 2 diverging).
pub fn scaled(scale: u32, x: f64, lo: f64, hi: f64) -> [u8; 3] {
    match scale {
        0 => along(&GRAY, fraction(x, lo, hi)),
        2 => {
            let m = lo.abs().max(hi.abs());
            along(&DIVERGING, fraction(x, -m, m))
        }
        _ => rgb(x, lo, hi),
    }
}

/// Dark ink on a light fill, light ink on a dark one.
pub fn ink_on([r, g, b]: [u8; 3]) -> &'static str {
    let light = 0.299 * f64::from(r) + 0.587 * f64::from(g) + 0.114 * f64::from(b);
    if light > 140.0 { "#1f2937" } else { "#f8fafc" }
}

fn fraction(x: f64, lo: f64, hi: f64) -> f64 {
    if hi > lo {
        ((x - lo) / (hi - lo)).clamp(0.0, 1.0)
    } else {
        0.5
    }
}

/// The color a fraction t of the way along `stops`.
fn along(stops: &[(u8, u8, u8)], t: f64) -> [u8; 3] {
    let scaled = t * (stops.len() - 1) as f64;
    let i = (scaled.floor() as usize).min(stops.len() - 2);
    let f = scaled - i as f64;
    let mix = |a: u8, b: u8| (f64::from(a) + (f64::from(b) - f64::from(a)) * f).round() as u8;
    let (a, b) = (stops[i], stops[i + 1]);
    [mix(a.0, b.0), mix(a.1, b.1), mix(a.2, b.2)]
}
