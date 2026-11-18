//! N-wise reduction (B21, APL2's `N f/ V`): `n 'f n_wise v` reduces f
//! over every window of n consecutive major cells of v, each window
//! folded from the right as `r_/` folds, one call at a time (a kernel).

mod windows;

pub use windows::n_wise;
