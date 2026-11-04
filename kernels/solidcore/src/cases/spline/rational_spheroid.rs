//! A spheroid: the region of the xz plane between the z axis from (0, 0, −10) to (0, 0, 10) and the
//! rational quadratic B-spline of control points (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10),
//! (0, 0, 10), weighted 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ½, ½, 1, 1, 1), turned a whole
//! turn about the z axis.
use crate::shared::geometry::{axis, turned};
use crate::Refusal;
use solidcore_ops::sweep::Curve;
use solidcore_ops::Body;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<Body, Refusal> {
    let w = FRAC_1_SQRT_2;
    let meridian = Curve::Nurbs {
        degree: 2,
        points: vec![
            [0.0, 0.0, -10.0, 1.0],
            [6.0, 0.0, -10.0, w],
            [6.0, 0.0, 0.0, 1.0],
            [6.0, 0.0, 10.0, w],
            [0.0, 0.0, 10.0, 1.0],
        ],
        knots: vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
    };
    turned(vec![meridian, axis(10.0, -10.0)])
}
