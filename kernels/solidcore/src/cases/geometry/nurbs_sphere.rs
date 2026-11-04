//! A ball of radius 5 centred on the origin, its whole surface one NURBS face: the standard
//! biquadratic sphere, the tensor product of the unit circle's nine control points cᵢ round the
//! axis (u) and the semicircle's five (ρⱼ, zⱼ) from the south pole to the north (v), control
//! point (i, j) at (ρⱼcᵢ, zⱼ) weighted wᵢvⱼ. Its edges v = 0 and v = 1 are the poles, and u = 0
//! and u = 1 the same meridian: the face, sewn to itself along that seam, closes the ball.
use crate::shared::nurbs::{rational_spline, sewn, Face};
use crate::Refusal;
use solidcore_ops::Body;
use std::f64::consts::FRAC_1_SQRT_2;

/// The unit circle's control points cᵢ.
const C: [[f64; 2]; 9] =
    [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [-1.0, 1.0], [-1.0, 0.0], [-1.0, -1.0], [0.0, -1.0], [1.0, -1.0], [1.0, 0.0]];
/// The meridian's control points (ρⱼ, zⱼ).
const M: [[f64; 2]; 5] = [[0.0, -5.0], [5.0, -5.0], [5.0, 0.0], [5.0, 5.0], [0.0, 5.0]];

pub fn build() -> Result<Body, Refusal> {
    let w = |i: usize| if i % 2 == 1 { FRAC_1_SQRT_2 } else { 1.0 };
    let net: Vec<Vec<[f64; 4]>> = (0..9)
        .map(|i| (0..5).map(|j| [M[j][0] * C[i][0], M[j][0] * C[i][1], M[j][1], w(i) * w(j)]).collect())
        .collect();
    let u_knots = [0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];
    let v_knots = [0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0];
    sewn(&[Face::Surface(rational_spline(&u_knots, &v_knots, &net))])
}
