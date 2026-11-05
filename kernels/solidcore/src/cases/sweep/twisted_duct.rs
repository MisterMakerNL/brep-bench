//! A duct along the y axis from the square A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10),
//! D (10, 0, 10) at y = 0 to the diamond A′ (0, 40, −12), B′ (−12, 40, 0), C′ (0, 40, 12),
//! D′ (12, 40, 0) at y = 40, each corner running straight to its namesake: a ruled loft.
use crate::bench::polygon;
use crate::Refusal;
use solidcore_ops::sweep::loft;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let square = polygon(&[[10.0, 0.0, -10.0], [-10.0, 0.0, -10.0], [-10.0, 0.0, 10.0], [10.0, 0.0, 10.0]]);
    let diamond = polygon(&[[0.0, 40.0, -12.0], [-12.0, 40.0, 0.0], [0.0, 40.0, 12.0], [12.0, 40.0, 0.0]]);
    // degree 1 across the sections: each side of the square ruled to the diamond's side
    Ok(loft(&[square, diamond], 1, true)?)
}
