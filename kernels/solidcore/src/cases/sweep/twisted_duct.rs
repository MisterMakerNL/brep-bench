//! A solid duct along the y axis from y = 0 to y = 40: the square A (10, 0, −10), B (−10, 0, −10),
//! C (−10, 0, 10), D (10, 0, 10) at its start and the diamond A′ (0, 40, −12), B′ (−12, 40, 0),
//! C′ (0, 40, 12), D′ (12, 40, 0) at its end, each corner running straight to its namesake, each
//! long face the twisted ruled surface between two neighbouring corner lines; its ends flat.
use crate::bench::polygon;
use solidcore_ops::sweep::loft;
use solidcore_ops::{Body, Result};

pub fn build() -> Result<Body> {
    let square =
        polygon(&[[10.0, 0.0, -10.0], [-10.0, 0.0, -10.0], [-10.0, 0.0, 10.0], [10.0, 0.0, 10.0]]);
    let diamond =
        polygon(&[[0.0, 40.0, -12.0], [-12.0, 40.0, 0.0], [0.0, 40.0, 12.0], [12.0, 40.0, 0.0]]);
    // skinned straight across (degree 1): each side AB of the square ruled to A′B′ of the diamond
    loft(&[square, diamond], 1, true)
}
