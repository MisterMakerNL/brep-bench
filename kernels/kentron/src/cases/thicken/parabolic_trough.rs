//! A sheet, the parabolic cylinder z = x²/8 for −4 ≤ x ≤ 4 and 0 ≤ y ≤ 10, made as the B-spline
//! surface of degree 2 by 1 the description gives (poles (−4, y, 2), (0, y, −2), (4, y, 2) for
//! y = 0 and 10, no interior knots), thickened by 1 to its concave side, toward the focus.
use crate::bench::p;
use crate::shared::thicken::thickened;
use crate::Refusal;
use kentron::geometry::surface::{BSplineSurface, KnotVector, Surface};
use kentron::make;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let section = [(-4.0, 2.0), (0.0, -2.0), (4.0, 2.0)];
    let poles = section.iter().map(|&(x, z)| vec![p(x, 0.0, z), p(x, 10.0, z)]).collect();
    let u = KnotVector::new(vec![0.0, 1.0], vec![3, 3], 2);
    let v = KnotVector::new(vec![0.0, 1.0], vec![2, 2], 1);
    let trough = Surface::BSpline(BSplineSurface::try_new(poles, u, v)?);
    // ∂S/∂u × ∂S/∂v is +z at the vertex: the normal points to the concave side
    let sheet = make::face_on_surface(m, trough)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
