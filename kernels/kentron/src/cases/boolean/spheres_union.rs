//! Two balls of radius 5, one centred on the origin and one at (6, 0, 0): their union.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(5.0).build(m)?;
    let second = Sphere::new(5.0).with_frame(frame(p(6.0, 0.0, 0.0), Direction::Z)).build(m)?;
    boolean::union(m, first, second)
}
