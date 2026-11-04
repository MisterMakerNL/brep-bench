//! What the solidcore implementations of brep-bench share: curves written briefly, the booleans
//! as results, rigid motions, and a face or an edge picked by a point on it, as a description
//! names one by where it is.
//!
//! solidcore-ops names faces and edges by their index in the body's record
//! ([`Body::to_record`]), and its operations take those indices with the record's [`Order`]. A
//! pick is the kernel's: an edge is the one whose closest point to the given point
//! ([`Body::edge_closest_point`]) is on it, and a face is the kernel's `SmApiFindFaces` of the
//! point.
use crate::Refusal;
use solidcore::kernel::Kernel;
use solidcore::query::api_queries::find_faces;
use solidcore::topology::brep_cache::arena_identity;
use solidcore::{BrepId, Point3};
use solidcore_ops::sweep::Curve;
use solidcore_ops::{boolean::boolean, Body, BooleanOp, Order};
use std::f64::consts::TAU;

/// How near a point must be to an edge to pick it.
pub const PICK_TOLERANCE: f64 = 1e-6;

/// `a` and `b` united.
pub fn fuse(a: Body, b: Body) -> Result<Body, Refusal> {
    boolean(a, b, BooleanOp::Union)?.ok_or_else(|| "the union is empty".into())
}

/// `a` less `b`.
pub fn cut(a: Body, b: Body) -> Result<Body, Refusal> {
    boolean(a, b, BooleanOp::Subtract)?.ok_or_else(|| "the difference is empty".into())
}

/// What is in both `a` and `b`.
pub fn common(a: Body, b: Body) -> Result<Body, Refusal> {
    boolean(a, b, BooleanOp::Intersect)?.ok_or_else(|| "the intersection is empty".into())
}

/// `body` less each of `tools`, one after another.
pub fn cut_all(body: Body, tools: impl IntoIterator<Item = Body>) -> Result<Body, Refusal> {
    tools.into_iter().try_fold(body, cut)
}

/// The union of `bodies`, one after another.
pub fn fuse_all(bodies: impl IntoIterator<Item = Body>) -> Result<Body, Refusal> {
    let mut it = bodies.into_iter();
    let first = it.next().ok_or("nothing to unite")?;
    it.try_fold(first, fuse)
}

/// The segment from `a` to `b`.
pub fn line(a: [f64; 3], b: [f64; 3]) -> Curve {
    Curve::Line { start: a, end: b }
}

/// The closed polygon through `points`, in order.
pub fn polygon(points: &[[f64; 3]]) -> Vec<Curve> {
    (0..points.len()).map(|i| line(points[i], points[(i + 1) % points.len()])).collect()
}

/// The arc about the unit `axis` through `center`, of `radius`, from angle `start` to `end`
/// (radians, counter-clockwise about the axis), angle 0 along the unit `reference`.
pub fn arc(center: [f64; 3], axis: [f64; 3], reference: [f64; 3], radius: f64, start: f64, end: f64) -> Curve {
    Curve::Arc { center, axis, reference, radius, start, end }
}

/// The whole circle about the unit `axis` through `center`, of `radius`, starting along the
/// unit `reference`.
pub fn circle(center: [f64; 3], axis: [f64; 3], reference: [f64; 3], radius: f64) -> Curve {
    arc(center, axis, reference, radius, 0.0, TAU)
}

/// The affine matrix ops takes (rows 0-2 the images of x, y and z, row 3 the translation) of the
/// turn by `angle` radians about the axis through `origin` along the unit `axis`, right-handed,
/// then the move by `shift`.
pub fn motion(origin: [f64; 3], axis: [f64; 3], angle: f64, shift: [f64; 3]) -> [[f64; 4]; 4] {
    let [x, y, z] = axis;
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    // the image of each unit vector e under the rotation, as a row
    let r = [
        [t * x * x + c, t * x * y + s * z, t * x * z - s * y],
        [t * x * y - s * z, t * y * y + c, t * y * z + s * x],
        [t * x * z + s * y, t * y * z - s * x, t * z * z + c],
    ];
    // p' = (p - o) R + o + shift
    let mut row3 = [0.0; 3];
    for j in 0..3 {
        row3[j] = origin[j] + shift[j] - (0..3).map(|i| origin[i] * r[i][j]).sum::<f64>();
    }
    [
        [r[0][0], r[0][1], r[0][2], 0.0],
        [r[1][0], r[1][1], r[1][2], 0.0],
        [r[2][0], r[2][1], r[2][2], 0.0],
        [row3[0], row3[1], row3[2], 1.0],
    ]
}

/// `body` moved by `v`.
pub fn translated(body: Body, v: [f64; 3]) -> Result<Body, Refusal> {
    body.transform(&motion([0.0; 3], [0.0, 0.0, 1.0], 0.0, v))?;
    Ok(body)
}

/// `body` turned by `degrees` about the axis through `origin` along the unit `axis`.
pub fn rotated(body: Body, origin: [f64; 3], axis: [f64; 3], degrees: f64) -> Result<Body, Refusal> {
    body.transform(&motion(origin, axis, degrees.to_radians(), [0.0; 3]))?;
    Ok(body)
}

/// The record order of `body`: what its faces and edges are numbered by.
pub fn order(body: &Body) -> Result<Order, Refusal> {
    Ok(body.to_record()?.1)
}

/// The record indices of the edges of `body` that `point` lies on.
pub fn edges_through(body: &Body, order: &Order, point: [f64; 3]) -> Result<Vec<usize>, Refusal> {
    let mut found = Vec::new();
    for i in 0..order.edges.len() {
        if body.edge_closest_point(order, i, point)?.is_some_and(|c| c.distance < PICK_TOLERANCE) {
            found.push(i);
        }
    }
    Ok(found)
}

/// The record index of the one edge of `body` that `point` lies on.
pub fn edge_through(body: &Body, order: &Order, point: [f64; 3]) -> Result<usize, Refusal> {
    match edges_through(body, order, point)?[..] {
        [e] => Ok(e),
        ref found => Err(format!("{} edges through {point:?}", found.len()).into()),
    }
}

/// The record indices of the edges of `body` through each of `points`, one edge each.
pub fn edges_through_each(body: &Body, order: &Order, points: &[[f64; 3]]) -> Result<Vec<usize>, Refusal> {
    points.iter().map(|&p| edge_through(body, order, p)).collect()
}

/// Every edge of `body`, by record index.
pub fn all_edges(order: &Order) -> Vec<usize> {
    (0..order.edges.len()).collect()
}

/// The record index of the face of `body` that `point` lies on (the kernel's `SmApiFindFaces`,
/// on the body's record made the kernel's again, whose order is the record's).
pub fn face_through(body: &Body, point: [f64; 3]) -> Result<usize, Refusal> {
    let (record, _) = body.to_record()?;
    let mut k = Kernel::default();
    let (mut brep, order) = solidcore_convert::brep::to_kernel(&mut k, &record, true)
        .map_err(|e| format!("the record does not convert: {e:?}"))?;
    let p = Point3::new(point[0], point[1], point[2]);
    let face = find_faces(&k, &mut brep, BrepId::from_index(0), p, &arena_identity, &|_| None)
        .map_err(|e| format!("no face through {point:?}: {e}"))?;
    order.faces.iter().position(|&f| f == face).ok_or_else(|| format!("no face through {point:?}").into())
}

/// The record indices of the faces of `body` through each of `points`, one face each.
pub fn faces_through_each(body: &Body, points: &[[f64; 3]]) -> Result<Vec<usize>, Refusal> {
    points.iter().map(|&p| face_through(body, p)).collect()
}
