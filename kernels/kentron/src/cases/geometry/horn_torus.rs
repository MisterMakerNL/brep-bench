//! A solid horn torus about the z axis: the middle circle of its tube has radius 5 and the tube
//! has radius 5 too, so its hole closes to the origin. It is the disc (x − 5)² + z² ≤ 25 of the
//! xz plane turned a whole turn about the z axis, as the description gives it (kentron's torus
//! primitive refuses a tube as wide as its middle circle; the revolution makes the same torus).
use crate::bench::p;
use crate::shared::geometry::{flat, turned};
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let frame = Frame::new(p(5.0, 0.0, 0.0), -Direction::Y, Direction::X);
    let rim = make::edge_from_curve(m, Curve::Circle(Circle::new(frame, 5.0)))?;
    let disc = flat(m, &[rim])?;
    turned(m, disc)
}
