//! A prism 10 tall on the xy plane, from z = 0 to z = 10, whose section is the region inside the
//! closed, uniform, periodic cubic B-spline with the twelve control points (14, 0), (12, 8),
//! (4, 6), (0, −21/8), (−4, 6), (−12, 8), (−14, 0), (−12, −8), (−4, −6), (0, 21/8), (4, −6),
//! (12, −8): a peanut with a waist 0.5 wide.
//!
//! cadrs's sketch holds no B-spline, so the curve enters the kernel as a wire (`create_wire`)
//! and its face is its Fill, extruded 10 up. The kernel's B-splines carry no periodic flag; the
//! periodic curve is the B-spline whose poles are the twelve control points followed again by
//! the first three, on uniform knots 0, 1, …, 18, over its range from knot 3 to knot 15: each of
//! its twelve spans the uniform cubic segment of four consecutive control points, wrapping
//! around, as the description defines the curve.
use crate::bench::{p3, v3, xy};
use crate::shared::spline::{extrude_face, filled};
use crate::Refusal;
use cadrs_kernel::{BodyId, Bspline3, DefaultKernel, PathCurve};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let points = [
        (14.0, 0.0),
        (12.0, 8.0),
        (4.0, 6.0),
        (0.0, -21.0 / 8.0),
        (-4.0, 6.0),
        (-12.0, 8.0),
        (-14.0, 0.0),
        (-12.0, -8.0),
        (-4.0, -6.0),
        (0.0, 21.0 / 8.0),
        (4.0, -6.0),
        (12.0, -8.0),
    ];
    let poles = (0..points.len() + 3)
        .map(|i| {
            let (x, y) = points[i % points.len()];
            p3(x, y, 0.0)
        })
        .collect::<Vec<_>>();
    let knots = (0..poles.len() + 4).map(|i| i as f64).collect();
    let curve = PathCurve::Bspline(Bspline3 { degree: 3, knots, poles, weights: vec![] });
    let face = filled(k, &[curve])?;
    extrude_face(k, xy(0.0), face, v3(0.0, 0.0, 1.0), 10.0)
}
