//! What the boolean implementations share: a half-space bounded by a plane.
use kentron::geometry::surface::{Plane, Surface};
use kentron::math::{Direction, Point};
use kentron::topology::{Model, Shape};
use kentron::{make, primitives};

/// How far the plane's face reaches from `origin` each way: past everything the cases cut.
const REACH: f64 = 1000.0;

/// The half-space bounded by the plane through `origin` with normal `normal`, on the side of it
/// where `reference` lies: kentron's half-space solid on a face of that plane.
pub fn half_space(m: &mut Model, origin: Point, normal: Direction, reference: Point) -> kentron::Result<Shape> {
    let plane = Surface::Plane(Plane::new(origin, normal));
    let face = make::face_on_domain(m, plane, -REACH, REACH, -REACH, REACH)?;
    primitives::half_space(m, face, reference)
}
