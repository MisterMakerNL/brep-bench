//! What the loft implementations share: polygons, the flat faces they bound, and the region
//! between a guide seen side-on and its mirror image, slid across.
use kentron::geometry::curve::{BezierCurve, Curve};
use kentron::make;
use kentron::math::{Point, Vector};
use kentron::sweep;
use kentron::topology::{Model, Shape};

/// The closed polygon through `corners`, in order.
pub fn polygon(m: &mut Model, corners: &[Point]) -> kentron::Result<Shape> {
    let mut edges = Vec::with_capacity(corners.len());
    for (k, &a) in corners.iter().enumerate() {
        edges.push(make::edge_between(m, a, corners[(k + 1) % corners.len()])?);
    }
    make::wire(m, &edges)
}

/// The flat face bounded by the polygon through `corners`.
pub fn flat(m: &mut Model, corners: &[Point]) -> kentron::Result<Shape> {
    let wire = polygon(m, corners)?;
    make::planar_face(m, wire)
}

/// The Bézier curve on `poles`, as an edge.
pub fn bezier(m: &mut Model, poles: &[Point]) -> kentron::Result<Shape> {
    make::edge_from_curve(m, Curve::Bezier(BezierCurve::new(poles.to_vec())))
}

/// Everything within the guide `curve` (a Bézier curve in a plane through the z axis, rising)
/// of the z axis, across that plane: the region between the curve and its mirror image in the
/// z axis (`mirror` maps a point to it), closed by a segment at the foot and another at the
/// top (unless the curves meet there), extruded along `across` from −`across` to +`across`.
pub fn between(m: &mut Model, curve: &[Point], mirror: impl Fn(Point) -> Point, across: Vector) -> kentron::Result<Shape> {
    let image: Vec<Point> = curve.iter().rev().map(|&q| mirror(q)).collect();
    let (top, foot) = (*curve.last().expect("a curve"), curve[0]);
    let mut edges = vec![bezier(m, curve)?];
    if top.distance(mirror(top)) > 1e-9 {
        edges.push(make::edge_between(m, top, mirror(top))?);
    }
    edges.push(bezier(m, &image)?);
    edges.push(make::edge_between(m, mirror(foot), foot)?);
    let wire = make::wire(m, &edges)?;
    let face = make::planar_face(m, wire)?;
    let face = face.moved(&kentron::math::Transform::translation(-across));
    Ok(sweep::extrude(m, face, across * 2.0)?.shape)
}
