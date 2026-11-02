//! A napkin ring, a ball of radius 5 about the origin with a hole of radius 3 bored through it
//! along the z axis, grown by 1: every point within 1 of it, its rims pieces of tori. The drill
//! runs from z = −10 to z = 10, past the ball at both ends.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(5.0).build(m)?;
    let drill = Cylinder::new(3.0, 20.0).with_frame(frame(p(0.0, 0.0, -10.0), Direction::Z)).build(m)?;
    let ring = boolean::difference(m, ball, drill)?;
    offset::shape(m, ring, 1.0, Join::Arc)
}
