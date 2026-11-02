//! A prism from z = 0 to z = 10 whose section is the periodic uniform quadratic B-spline,
//! non-rational, on the control square (10, 10), (−10, 10), (−10, −10), (10, −10), closing on
//! itself: a square rounded all the way to the middle of each side. Its face extruded straight
//! up.
use crate::bench::{p, v};
use crate::shared::geometry::flat;
use kentron::geometry::curve::{BSplineCurve, Curve};
use kentron::make;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // a periodic curve has as many poles as the polygon has corners, its knots 0 … 4 each once
    let corners = vec![p(10.0, 10.0, 0.0), p(-10.0, 10.0, 0.0), p(-10.0, -10.0, 0.0), p(10.0, -10.0, 0.0)];
    let curve = BSplineCurve::try_new_periodic(corners, vec![0.0, 1.0, 2.0, 3.0, 4.0], vec![1; 5], 2)?;
    let rim = make::edge_from_curve(m, Curve::BSpline(curve))?;
    let end = flat(m, &[rim])?;
    Ok(sweep::extrude(m, end, v(0.0, 0.0, 10.0))?.shape)
}
