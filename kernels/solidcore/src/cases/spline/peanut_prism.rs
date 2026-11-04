//! A prism from z = 0 to z = 10 on a peanut: the closed, uniform, periodic cubic B-spline of
//! twelve control points in the plane z = 0, extruded straight up, capped. The kernel holds
//! clamped B-splines only: it clamps the periodic curve itself.
use crate::shared::nurbs::periodic;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

const POLYGON: [[f64; 3]; 12] = [
    [14.0, 0.0, 0.0],
    [12.0, 8.0, 0.0],
    [4.0, 6.0, 0.0],
    [0.0, -21.0 / 8.0, 0.0],
    [-4.0, 6.0, 0.0],
    [-12.0, 8.0, 0.0],
    [-14.0, 0.0, 0.0],
    [-12.0, -8.0, 0.0],
    [-4.0, -6.0, 0.0],
    [0.0, 21.0 / 8.0, 0.0],
    [4.0, -6.0, 0.0],
    [12.0, -8.0, 0.0],
];

pub fn build() -> Result<Body, Refusal> {
    let section = periodic(3, &POLYGON)?;
    Ok(extrude(&[section], [0.0, 0.0, 1.0], 10.0, 0.0, DraftCorner::Curvature, true)?)
}
