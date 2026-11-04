//! A hollow square tube tapering along the y axis from y = 0 to y = 40: its outside a square
//! 24 × 24 at y = 0 and 12 × 12 at y = 40, its hole 20 × 20 and 8 × 8, the sizes running
//! linearly between, so its eight long faces are flat trapezoids; its ends flat.
use crate::bench::{cut, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{loft, Curve};
use solidcore_ops::Body;

/// The square of half-width `h` centred on the y axis in the plane at `y`, sides along x and z.
fn square(y: f64, h: f64) -> Vec<Curve> {
    polygon(&[[-h, y, -h], [h, y, -h], [h, y, h], [-h, y, h]])
}

pub fn build() -> Result<Body, Refusal> {
    // each the skin, straight across, between its two end squares, capped
    let outside = loft(&[square(0.0, 12.0), square(40.0, 6.0)], 1, true)?;
    let hole = loft(&[square(0.0, 10.0), square(40.0, 4.0)], 1, true)?;
    cut(outside, hole)
}
