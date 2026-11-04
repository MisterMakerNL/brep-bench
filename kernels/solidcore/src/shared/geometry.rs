//! What the solidcore implementations of the geometry cases share.
use crate::bench::line;
use crate::shared::nurbs::{bezier_knots, spline, split_keeping};
use crate::Refusal;
use solidcore_ops::split::Tool;
use solidcore_ops::sweep::{extrude, revolve, Curve, DraftCorner};
use solidcore_ops::{make_cylinder, make_sphere, Body};
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

/// The ellipsoid x²/a² + y²/b² + z²/c² ≤ 1: solidcore has no ellipsoid, so a ball of radius 1
/// stretched by a, b and c along x, y and z (the kernel turns its sphere into a NURBS).
pub fn ellipsoid(a: f64, b: f64, c: f64) -> Result<Body, Refusal> {
    let ball = make_sphere([0.0; 3], 1.0)?;
    ball.transform(&[[a, 0.0, 0.0, 0.0], [0.0, b, 0.0, 0.0], [0.0, 0.0, c, 0.0], [0.0, 0.0, 0.0, 1.0]])?;
    Ok(ball)
}

/// The ellipse of semi-axes `a` along x and `b` along y about the z axis in the plane z = `z`,
/// as the rational quadratic it is: the nine control points of the square about it, weighted
/// 1 at its sides' middles and √2/2 at its corners, on the knots of four quarters.
pub fn ellipse(a: f64, b: f64, z: f64) -> Curve {
    let corners = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];
    let points = corners
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| [a * x, b * y, z, if i % 2 == 1 { FRAC_1_SQRT_2 } else { 1.0 }])
        .collect();
    Curve::Nurbs { degree: 2, points, knots: vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0] }
}

/// The ellipse of [`ellipse`] in the plane z = `z0` extruded straight up by `height`.
pub fn elliptic_cylinder(a: f64, b: f64, z0: f64, height: f64) -> Result<Body, Refusal> {
    Ok(extrude(&[ellipse(a, b, z0)], [0.0, 0.0, 1.0], height, 0.0, DraftCorner::Curvature, true)?)
}

/// The region of the xz plane the closed `profile` bounds (on the side x ≥ 0, touching or
/// running along the z axis or clear of it), turned a whole turn about the z axis, capped.
pub fn turned(profile: Vec<Curve>) -> Result<Body, Refusal> {
    Ok(revolve(&profile, [0.0; 3], [0.0, 0.0, 1.0], TAU, true)?)
}

/// The segment of the z axis from height `a` to height `b`.
pub fn axis(a: f64, b: f64) -> Curve {
    line([0.0, 0.0, a], [0.0, 0.0, b])
}

/// The drum of `geometry/saddle-drum`: a cylinder of radius 8 about the z axis from z = 0 to
/// z = 20, above the saddle everywhere, split by the bilinear B-spline patch through
/// (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and (10, 10, 22.5) (the square
/// −10 ≤ x, y ≤ 10, reaching past the cylinder), and the part below it kept.
pub fn saddle_drum() -> Result<Body, Refusal> {
    let saddle = spline(
        &bezier_knots(1),
        &bezier_knots(1),
        vec![
            vec![[-10.0, -10.0, 22.5], [-10.0, 10.0, -2.5]],
            vec![[10.0, -10.0, -2.5], [10.0, 10.0, 22.5]],
        ],
    );
    let tool = Tool { surface: saddle, domain: [[0.0, 1.0], [0.0, 1.0]] };
    let drum = make_cylinder([0.0; 3], 8.0, 20.0)?;
    split_keeping(&drum, &tool, [0.0, 0.0, 1.0])
}

/// The quadratic Bézier from `p0` to `p2` about `p1`, its middle point weighted `w`: a conic.
pub fn conic(p0: [f64; 3], p1: [f64; 3], w: f64, p2: [f64; 3]) -> Curve {
    Curve::Nurbs {
        degree: 2,
        points: vec![[p0[0], p0[1], p0[2], 1.0], [p1[0], p1[1], p1[2], w], [p2[0], p2[1], p2[2], 1.0]],
        knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    }
}
