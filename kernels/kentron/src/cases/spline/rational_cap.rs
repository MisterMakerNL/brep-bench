//! A solid turned a full turn about the z axis from the region in the xz plane (x ≥ 0) bounded by
//! the z axis from (0, 0, −5) to (0, 0, 10), the segment from (0, 0, −5) to (5√3, 0, −5), and the
//! rational quadratic Bézier curve from (5√3, 0, −5) to (0, 0, 10) with control points
//! (5√3, 0, −5), (10√3, 0, 10), (0, 0, 10) and weights 1, ½, 1: an arc of the circle of radius 10
//! about the origin. So it is a ball's cap standing on a disc at z = −5.
use crate::bench::p;
use kentron::geometry::curve::{BezierCurve, Curve};
use kentron::make;
use kentron::math::Axis;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let r3 = 3f64.sqrt();
    let (rim, top, bottom) = (p(5.0 * r3, 0.0, -5.0), p(0.0, 0.0, 10.0), p(0.0, 0.0, -5.0));
    let arc = BezierCurve::rational(vec![rim, p(10.0 * r3, 0.0, 10.0), top], vec![1.0, 0.5, 1.0]);
    let arc = make::edge_from_curve(m, Curve::Bezier(arc))?;
    let axis = make::edge_between(m, top, bottom)?;
    let floor = make::edge_between(m, bottom, rim)?;
    let outline = make::wire(m, &[arc, axis, floor])?;
    let region = make::planar_face(m, outline)?;
    Ok(sweep::revolve(m, region, Axis::Z, TAU)?.shape)
}
