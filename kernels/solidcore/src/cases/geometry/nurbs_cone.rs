//! A cone on the disc of radius 5 about the z axis in z = 0, its apex at (0, 0, 10): its side one
//! NURBS surface, rational quadratic round the axis (u, the circle's nine control points 5cᵢ and
//! weights wᵢ) and linear up it (v, to nine copies of the apex, weighted wᵢ), with a pole at the
//! apex; its base the flat disc inside that circle. The two faces, sewn into a solid.
use crate::shared::nurbs::{bezier_knots, rational_curve, rational_spline, sewn, Face};
use crate::Refusal;
use solidcore_ops::Body;
use std::f64::consts::FRAC_1_SQRT_2;

/// The unit circle's control points cᵢ.
const C: [[f64; 2]; 9] =
    [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [-1.0, 1.0], [-1.0, 0.0], [-1.0, -1.0], [0.0, -1.0], [1.0, -1.0], [1.0, 0.0]];
const CIRCLE_KNOTS: [f64; 12] = [0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];

pub fn build() -> Result<Body, Refusal> {
    let w = |i: usize| if i % 2 == 1 { FRAC_1_SQRT_2 } else { 1.0 };
    let rim: Vec<[f64; 4]> = (0..9).map(|i| [5.0 * C[i][0], 5.0 * C[i][1], 0.0, w(i)]).collect();
    let net: Vec<Vec<[f64; 4]>> = (0..9).map(|i| vec![rim[i], [0.0, 0.0, 10.0, w(i)]]).collect();
    sewn(&[
        Face::Surface(rational_spline(&CIRCLE_KNOTS, &bezier_knots(1), &net)),
        Face::Flat(vec![rational_curve(&CIRCLE_KNOTS, &rim)]),
    ])
}
