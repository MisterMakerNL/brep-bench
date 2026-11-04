//! A ring about the z axis: the closed, periodic uniform quadratic B-spline of the xz plane whose
//! control polygon is the square (13, 0, 3), (7, 0, 3), (7, 0, −3), (13, 0, −3), turned a whole
//! turn about the z axis. The kernel holds clamped B-splines only: it clamps the periodic curve
//! itself.
use crate::shared::geometry::turned;
use crate::shared::nurbs::periodic;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let section = periodic(2, &[[13.0, 0.0, 3.0], [7.0, 0.0, 3.0], [7.0, 0.0, -3.0], [13.0, 0.0, -3.0]])?;
    turned(vec![section])
}
