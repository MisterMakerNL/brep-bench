//! A solid turned a full turn about the z axis from the region in the xz plane (x ≥ 0) between the
//! z axis from (0, 0, −10) to (0, 0, 10) and the rational quadratic B-spline curve with control
//! points (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10), (0, 0, 10), weights 1, √2/2, 1, √2/2, 1
//! and knots (0, 0, 0, ½, ½, 1, 1, 1): two quarters of the ellipse x²/36 + z²/100 = 1. So it is a
//! spheroid 12 wide and 20 tall.
use crate::bench::p;
use kentron::geometry::curve::{BSplineCurve, Curve};
use kentron::make;
use kentron::math::Axis;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (bottom, top) = (p(0.0, 0.0, -10.0), p(0.0, 0.0, 10.0));
    let poles = vec![bottom, p(6.0, 0.0, -10.0), p(6.0, 0.0, 0.0), p(6.0, 0.0, 10.0), top];
    let weights = vec![1.0, FRAC_1_SQRT_2, 1.0, FRAC_1_SQRT_2, 1.0];
    // the knot vector (0, 0, 0, ½, ½, 1, 1, 1)
    let curve = BSplineCurve::try_new(poles, vec![0.0, 0.5, 1.0], vec![3, 2, 3], 2)?.with_weights(weights);
    let side = make::edge_from_curve(m, Curve::BSpline(curve))?;
    let axis = make::edge_between(m, top, bottom)?;
    let outline = make::wire(m, &[side, axis])?;
    let region = make::planar_face(m, outline)?;
    Ok(sweep::revolve(m, region, Axis::Z, TAU)?.shape)
}
