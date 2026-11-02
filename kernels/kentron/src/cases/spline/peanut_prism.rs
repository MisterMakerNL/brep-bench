//! A prism 10 tall standing on the xy plane, from z = 0 to z = 10, whose section is a peanut: the
//! region inside the closed, uniform, periodic cubic B-spline curve in the plane z = 0 with the
//! twelve control points (14, 0), (12, 8), (4, 6), (0, −21/8), (−4, 6), (−12, 8), (−14, 0),
//! (−12, −8), (−4, −6), (0, 21/8), (4, −6), (12, −8), wrapping round. Its waist is only 0.5 wide;
//! the prism's side is a single smooth face.
use crate::bench::{p, v};
use kentron::geometry::curve::{BSplineCurve, Curve};
use kentron::make;
use kentron::sweep;
use kentron::topology::{Model, Shape};

const CONTROL: [(f64, f64); 12] = [
    (14.0, 0.0), (12.0, 8.0), (4.0, 6.0), (0.0, -21.0 / 8.0), (-4.0, 6.0), (-12.0, 8.0),
    (-14.0, 0.0), (-12.0, -8.0), (-4.0, -6.0), (0.0, 21.0 / 8.0), (4.0, -6.0), (12.0, -8.0),
];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // periodic: one knot more than poles, evenly spaced, each simple
    let poles = CONTROL.iter().map(|&(x, y)| p(x, y, 0.0)).collect();
    let n = CONTROL.len();
    let knots = (0..=n).map(|k| k as f64).collect();
    let curve = BSplineCurve::try_new_periodic(poles, knots, vec![1; n + 1], 3)?;
    let rim = make::edge_from_curve(m, Curve::BSpline(curve))?;
    let rim = make::wire(m, &[rim])?;
    let section = make::planar_face(m, rim)?;
    Ok(sweep::extrude(m, section, v(0.0, 0.0, 10.0))?.shape)
}
