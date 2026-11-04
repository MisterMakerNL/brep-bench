//! A prism from z = 0 to z = 10 on the closed, periodic uniform quadratic B-spline whose control
//! polygon is the square (10, 10), (−10, 10), (−10, −10), (10, −10): the curve extruded straight
//! up, capped. The kernel holds clamped B-splines only: it clamps the periodic curve itself.
use crate::shared::nurbs::periodic;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let section = periodic(2, &[[10.0, 10.0, 0.0], [-10.0, 10.0, 0.0], [-10.0, -10.0, 0.0], [10.0, -10.0, 0.0]])?;
    Ok(extrude(&[section], [0.0, 0.0, 1.0], 10.0, 0.0, DraftCorner::Curvature, true)?)
}
