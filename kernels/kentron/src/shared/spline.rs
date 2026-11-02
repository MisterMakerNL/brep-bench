//! What the spline implementations share.
use crate::bench::p;
use kentron::geometry::curve::{BezierCurve, Circle, Curve};
use kentron::geometry::ParametricCurve;
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};

/// The polynomial Bézier curve with these control points.
pub fn bezier(poles: &[(f64, f64, f64)]) -> Curve {
    Curve::Bezier(BezierCurve::new(poles.iter().map(|&(x, y, z)| p(x, y, z)).collect()))
}

/// The plane square to a curve at its start, as a frame there whose z is the curve's tangent.
pub fn square_to_start(curve: &Curve) -> Frame {
    let (start, tangent) = curve.d1(curve.first_parameter());
    Frame::from_z(start, Direction::from_vector(tangent))
}

/// A pipe along `curve` of circular section: the disc of `radius` centred on the curve's start
/// and square to it, swept along the curve.
pub fn round_pipe(m: &mut Model, curve: Curve, radius: f64) -> kentron::Result<Shape> {
    let rim = Circle::new(square_to_start(&curve), radius);
    let rim = make::edge_from_curve(m, Curve::Circle(rim))?;
    let rim = make::wire(m, &[rim])?;
    let disc = make::planar_face(m, rim)?;
    let path = make::edge_from_curve(m, curve)?;
    Ok(kentron::sweep::pipe(m, disc, path)?.shape)
}
