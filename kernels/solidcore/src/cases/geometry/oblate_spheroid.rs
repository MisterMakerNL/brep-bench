//! An oblate spheroid: the half ellipse x²/36 + z²/9 = 1, x ≥ 0, of the xz plane, closed along the
//! z axis, turned a whole turn about the z axis. The half ellipse is the rational quadratic it
//! is: two quarters, through (0, 0, −3), (6, 0, 0) and (0, 0, 3), the corners of the rectangle
//! about them weighted √2/2.
use crate::shared::geometry::{axis, turned};
use crate::Refusal;
use solidcore_ops::sweep::Curve;
use solidcore_ops::Body;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<Body, Refusal> {
    let w = FRAC_1_SQRT_2;
    let half = Curve::Nurbs {
        degree: 2,
        points: vec![
            [0.0, 0.0, -3.0, 1.0],
            [6.0, 0.0, -3.0, w],
            [6.0, 0.0, 0.0, 1.0],
            [6.0, 0.0, 3.0, w],
            [0.0, 0.0, 3.0, 1.0],
        ],
        knots: vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
    };
    turned(vec![half, axis(3.0, -3.0)])
}
