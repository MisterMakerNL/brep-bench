//! The parabolic cylinder z = x²/8 for −4 ≤ x ≤ 4 and 0 ≤ y ≤ 10, a sheet of one B-spline face
//! (degree 2 in u through (−4, y, 2), (0, y, −2), (4, y, 2), degree 1 in v from y = 0 to 10),
//! thickened by 1 to its concave side, toward the focus.
use crate::shared::nurbs::{bezier_knots, sheet, spline, Face};
use crate::shared::thicken::thickened;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let row = |x: f64, z: f64| vec![[x, 0.0, z], [x, 10.0, z]];
    let surface = spline(&bezier_knots(2), &bezier_knots(1), vec![row(-4.0, 2.0), row(0.0, -2.0), row(4.0, 2.0)]);
    // the surface's natural normal, ∂S/∂u × ∂S/∂v, is +x × +y at the vertex: up, toward the focus
    thickened(sheet(&[Face::Surface(surface)])?, 1.0, false)
}
