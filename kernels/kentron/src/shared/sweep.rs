//! What the sweep implementations share: polylines and circles made into wires and faces.
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Direction, Frame, Point};
use kentron::topology::{Model, Shape};

/// The wire of straight edges through `points`, in order, closed back to the first if
/// `closed`.
pub fn polyline(m: &mut Model, points: &[Point], closed: bool) -> kentron::Result<Shape> {
    let vertices: Vec<Shape> = points.iter().map(|&p| make::vertex(m, p)).collect();
    let n = vertices.len();
    let count = if closed { n } else { n - 1 };
    let edges = (0..count).map(|i| make::edge(m, vertices[i], vertices[(i + 1) % n])).collect::<kentron::Result<Vec<_>>>()?;
    make::wire(m, &edges)
}

/// The closed wire of the circle of `radius` about `centre`, square to `normal`.
pub fn circle(m: &mut Model, centre: Point, normal: Direction, radius: f64) -> kentron::Result<Shape> {
    let edge = make::edge_from_curve(m, Curve::Circle(Circle::new(Frame::from_z(centre, normal), radius)))?;
    make::wire(m, &[edge])
}

/// The disc of `radius` about `centre`, square to `normal`.
pub fn disc(m: &mut Model, centre: Point, normal: Direction, radius: f64) -> kentron::Result<Shape> {
    let rim = circle(m, centre, normal, radius)?;
    make::planar_face(m, rim)
}

/// The planar face inside the closed polygon `outer` less the polygon `hole`.
pub fn holed_polygon(m: &mut Model, outer: &[Point], hole: &[Point]) -> kentron::Result<Shape> {
    let outer = polyline(m, outer, true)?;
    let face = make::planar_face(m, outer)?;
    let hole = polyline(m, hole, true)?;
    make::face_with_holes(m, face, &[hole])
}
