//! What the kentron implementations of brep-bench share: points, directions and frames written
//! briefly, and a face or an edge picked by a point on it, as a description names one by where it
//! is.
use crate::Refusal;
use kentron::{distance, make};
use kentron::math::{Direction, Frame, Point, Vector};
use kentron::topology::{Model, NodeKind, Shape};

/// A point.
pub fn p(x: f64, y: f64, z: f64) -> Point {
    Point::new(x, y, z)
}

/// A vector.
pub fn v(x: f64, y: f64, z: f64) -> Vector {
    Vector::new(x, y, z)
}

/// A direction (made unit).
pub fn d(x: f64, y: f64, z: f64) -> Direction {
    Direction::new(x, y, z)
}

/// A frame at `origin` with its z along `z`.
pub fn frame(origin: Point, z: Direction) -> Frame {
    Frame::from_z(origin, z)
}

/// How near a point must be to a face or edge to pick it.
pub const PICK_TOLERANCE: f64 = 1e-7;

/// The distinct sub-shapes of `kind` of `shape` that `point` lies on.
fn through(m: &mut Model, shape: Shape, kind: NodeKind, point: Point) -> Vec<Shape> {
    let vertex = make::vertex(m, point);
    let mut found: Vec<Shape> = Vec::new();
    for s in m.explore(shape, kind).collect::<Vec<_>>() {
        if found.iter().any(|f| f.node == s.node) {
            continue;
        }
        if distance::distance(m, vertex, s).is_ok_and(|d| d.value < PICK_TOLERANCE) {
            found.push(s);
        }
    }
    found
}

/// The one face of `shape` that `point` lies on.
pub fn face_through(m: &mut Model, shape: Shape, point: Point) -> Result<Shape, Refusal> {
    match &through(m, shape, NodeKind::Face, point)[..] {
        [f] => Ok(*f),
        found => Err(format!("{} faces through {point:?}", found.len()).into()),
    }
}

/// The one edge of `shape` that `point` lies on.
pub fn edge_through(m: &mut Model, shape: Shape, point: Point) -> Result<Shape, Refusal> {
    match &through(m, shape, NodeKind::Edge, point)[..] {
        [e] => Ok(*e),
        found => Err(format!("{} edges through {point:?}", found.len()).into()),
    }
}

/// Every edge of `shape`, each once.
pub fn edges(m: &Model, shape: Shape) -> Vec<Shape> {
    let mut found: Vec<Shape> = Vec::new();
    for e in m.explore(shape, NodeKind::Edge).collect::<Vec<_>>() {
        if !found.iter().any(|f| f.node == e.node) {
            found.push(e);
        }
    }
    found
}
