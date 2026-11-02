//! A ring about the z axis: the periodic uniform quadratic B-spline, non-rational, on the
//! control square (13, 0, 3), (7, 0, 3), (7, 0, −3), (13, 0, −3) of the xz plane, closing on
//! itself, turned a whole turn about the z axis.
use crate::bench::p;
use crate::shared::geometry::{flat, turned};
use kentron::geometry::curve::{BSplineCurve, Curve};
use kentron::make;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // a periodic curve has as many poles as the polygon has corners, its knots 0 … 4 each once
    let corners = vec![p(13.0, 0.0, 3.0), p(7.0, 0.0, 3.0), p(7.0, 0.0, -3.0), p(13.0, 0.0, -3.0)];
    let curve = BSplineCurve::try_new_periodic(corners, vec![0.0, 1.0, 2.0, 3.0, 4.0], vec![1; 5], 2)?;
    let rim = make::edge_from_curve(m, Curve::BSpline(curve))?;
    let section = flat(m, &[rim])?;
    turned(m, section)
}
