//! Helpers for the sweep, swept and loft cases: a sweep's path as one of brepkit's NURBS curves
//! (an arc, a polyline, a Bézier), and the flat profiles swept along it (a disc, a polygon, a
//! polygon with a polygonal hole).
use brepkit_geometry::convert::circle_to_nurbs;
use brepkit_math::curves::Circle3D;
use brepkit_math::nurbs::curve::NurbsCurve;
use brepkit_math::vec::{Point3, Vec3};
use brepkit_operations::OperationsError;
use brepkit_topology::builder::{
    make_circle_edge_with_ref, make_face_from_wire, make_polygon_wire,
};
use brepkit_topology::edge::{Edge, EdgeCurve};
use brepkit_topology::face::FaceId;
use brepkit_topology::vertex::Vertex;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

/// The tolerance the profiles' vertices are made with.
pub const TOL: f64 = 1e-7;

/// The arc of the circle about `center` in the plane square to `normal` through `start`, turning
/// `angle` radians right-handed about `normal`: brepkit's exact rational NURBS of it.
pub fn arc(
    center: Point3,
    normal: Vec3,
    start: Point3,
    angle: f64,
) -> Result<NurbsCurve, OperationsError> {
    let radial = start - center;
    let circle = Circle3D::new_with_ref(center, normal, radial.length(), radial)?;
    Ok(circle_to_nurbs(&circle, 0.0, angle)?)
}

/// The polyline through `points`, as a NURBS curve of degree 1 with a knot at each corner.
pub fn polyline(points: &[Point3]) -> Result<NurbsCurve, OperationsError> {
    let n = points.len();
    let mut knots = vec![0.0];
    knots.extend((0..n).map(|i| i as f64 / (n - 1) as f64));
    knots.push(1.0);
    Ok(NurbsCurve::new(1, knots, points.to_vec(), vec![1.0; n])?)
}

/// The Bézier curve on `points`, as a clamped NURBS curve of one span.
pub fn bezier(points: &[Point3]) -> Result<NurbsCurve, OperationsError> {
    let n = points.len();
    let mut knots = vec![0.0; n];
    knots.extend(vec![1.0; n]);
    Ok(NurbsCurve::new(
        n - 1,
        knots,
        points.to_vec(),
        vec![1.0; n],
    )?)
}

/// The disc of radius `r` about `center`, square to `normal`.
pub fn disc(
    topo: &mut Topology,
    center: Point3,
    normal: Vec3,
    r: f64,
) -> Result<FaceId, OperationsError> {
    // any direction across the normal will do: it only fixes where the circle's seam lies
    let across = if normal.x().abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let rim = make_circle_edge_with_ref(topo, center, normal, r, across, TOL)?;
    let wire = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, true)], true)?);
    Ok(make_face_from_wire(topo, wire)?)
}

/// The flat face bounded by the polygon through `corners`.
pub fn polygon(topo: &mut Topology, corners: &[Point3]) -> Result<FaceId, OperationsError> {
    let wire = make_polygon_wire(topo, corners, TOL)?;
    Ok(make_face_from_wire(topo, wire)?)
}

/// The flat face bounded by the polygon through `outer`, with the polygon through `hole` taken
/// out of it.
pub fn holed(
    topo: &mut Topology,
    outer: &[Point3],
    hole: &[Point3],
) -> Result<FaceId, OperationsError> {
    let face = polygon(topo, outer)?;
    // the hole runs round the other way from the outside
    let reversed: Vec<Point3> = hole.iter().rev().copied().collect();
    let hole = make_polygon_wire(topo, &reversed, TOL)?;
    topo.face_mut(face)?.inner_wires_mut().push(hole);
    Ok(face)
}

/// The flat face bounded by a closed chain of pieces, each running along its curve from its
/// point to the next piece's (the last back to the first).
pub fn chain(
    topo: &mut Topology,
    pieces: &[(Point3, EdgeCurve)],
) -> Result<FaceId, OperationsError> {
    let corners: Vec<_> = pieces
        .iter()
        .map(|(at, _)| topo.add_vertex(Vertex::new(*at, TOL)))
        .collect();
    let n = corners.len();
    let edges = pieces
        .iter()
        .enumerate()
        .map(|(i, (_, curve))| {
            let edge = topo.add_edge(Edge::new(corners[i], corners[(i + 1) % n], curve.clone()));
            OrientedEdge::new(edge, true)
        })
        .collect();
    let wire = topo.add_wire(Wire::new(edges, true)?);
    Ok(make_face_from_wire(topo, wire)?)
}

/// The stadium in the plane z = `z`: the rectangle |x − `cx`| ≤ `half`, |y − `cy`| ≤ `r`, with a
/// half disc of radius `r` on each of its ends, about (`cx` ± `half`, `cy`).
pub fn stadium(
    topo: &mut Topology,
    cx: f64,
    cy: f64,
    z: f64,
    half: f64,
    r: f64,
) -> Result<FaceId, OperationsError> {
    let up = Vec3::new(0.0, 0.0, 1.0);
    let at = |x: f64, y: f64| Point3::new(x, y, z);
    // each end's half circle, running anticlockwise about +z
    let end = |x: f64| Circle3D::new_with_ref(at(x, cy), up, r, Vec3::new(1.0, 0.0, 0.0));
    chain(
        topo,
        &[
            (at(cx - half, cy - r), EdgeCurve::Line),
            (at(cx + half, cy - r), EdgeCurve::Circle(end(cx + half)?)),
            (at(cx + half, cy + r), EdgeCurve::Line),
            (at(cx - half, cy + r), EdgeCurve::Circle(end(cx - half)?)),
        ],
    )
}
