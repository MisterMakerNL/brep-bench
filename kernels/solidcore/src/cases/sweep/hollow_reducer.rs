//! A hollow square tube tapering along the y axis: lofted, ruled, from the square 24 × 24 with a
//! hole 20 × 20 at y = 0 to the square 12 × 12 with a hole 8 × 8 at y = 40, all centered on the
//! y axis with sides parallel to x and z.
use crate::bench::polygon;
use crate::Refusal;
use solidcore_ops::sweep::{loft, Curve};
use solidcore_ops::Body;

/// The square of half-width `h` at `y`, centered on the y axis, its sides parallel to x and z.
fn square(h: f64, y: f64) -> Vec<Curve> {
    polygon(&[[h, y, -h], [-h, y, -h], [-h, y, h], [h, y, h]])
}

pub fn build() -> Result<Body, Refusal> {
    let start = [square(12.0, 0.0), square(10.0, 0.0)].concat();
    let end = [square(6.0, 40.0), square(4.0, 40.0)].concat();
    // degree 1 across the sections: each corner straight from one end to the other
    Ok(loft(&[start, end], 1, true)?)
}
