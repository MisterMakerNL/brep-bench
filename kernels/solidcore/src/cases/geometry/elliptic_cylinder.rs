//! The ellipse x²/25 + y²/9 ≤ 1 in the plane z = 0 extruded straight up to z = 10. ops's profile
//! curves are lines, arcs and NURBS, so the ellipse is the rational quadratic it is.
use crate::shared::geometry::elliptic_cylinder;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    elliptic_cylinder(5.0, 3.0, 0.0, 10.0)
}
