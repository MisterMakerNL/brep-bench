//! A square-to-round transition: the square −20 ≤ x, y ≤ 20 at z = 0 joined to the circle of
//! radius 15 about the z axis at z = 30 by four flat triangles (each side of the square to the
//! point of the circle nearest it) and four pieces of oblique cones (each corner of the square,
//! the apex, joined by straight lines to the quarter of the circle between the two points beside
//! it). solidcore's loft pairs the sections' curves one to one, so the faces are made and sewn
//! into a solid: each cone piece the NURBS surface ruled between its quarter circle (a rational
//! quadratic, the corner of the square about it weighted √2/2) and its apex.
use crate::shared::nurbs::{bezier_knots, polygon, rational_curve, rational_spline, sewn, Face};
use crate::Refusal;
use solidcore_ops::Body;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<Body, Refusal> {
    // the four points of the circle nearest the sides, from the −y side counter-clockwise, and
    // the corners of the square, the corner k between points k and k + 1
    let points = [[0.0, -15.0], [15.0, 0.0], [0.0, 15.0], [-15.0, 0.0]];
    let corners = [[20.0, -20.0], [20.0, 20.0], [-20.0, 20.0], [-20.0, -20.0]];
    let top = |[x, y]: [f64; 2], w: f64| [x, y, 30.0, w];
    let foot = |[x, y]: [f64; 2]| [x, y, 0.0];
    let mut faces = vec![Face::Flat(polygon(&corners.map(foot)))];
    let mut rim = Vec::new();
    for k in 0..4 {
        let (p, q) = (points[k], points[(k + 1) % 4]);
        // the quarter circle from p to q, about the corner (p + q) of the square round it
        let quarter = [top(p, 1.0), top([p[0] + q[0], p[1] + q[1]], FRAC_1_SQRT_2), top(q, 1.0)];
        let apex = foot(corners[k]);
        let net: Vec<Vec<[f64; 4]>> =
            quarter.iter().map(|&a| vec![a, [apex[0], apex[1], apex[2], a[3]]]).collect();
        faces.push(Face::Surface(rational_spline(&bezier_knots(2), &bezier_knots(1), &net)));
        rim.push(rational_curve(&bezier_knots(2), &quarter));
        // the triangle on the side before this corner, to point k
        faces.push(Face::Flat(polygon(&[foot(corners[(k + 3) % 4]), apex, [p[0], p[1], 30.0]])));
    }
    faces.push(Face::Flat(rim));
    sewn(&faces)
}
