//! A cube of side 10 between the origin and (10, 10, 10), and its inscribed ball, radius 5 about
//! (5, 5, 5), which touches each face at a single point. Their common part is the whole ball.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let ball = Sphere::new(5.0).with_frame(frame(p(5.0, 5.0, 5.0), Direction::Z)).build(m)?;
    boolean::intersection(m, cube, ball)
}
