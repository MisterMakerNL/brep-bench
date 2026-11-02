//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, with a ball of radius 5 centred at
//! (0, 0, 10) cut from it: the whole top face goes, and the hemispherical dish meets the wall
//! tangentially along the rim, a knife edge.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    let ball = Sphere::new(5.0).with_frame(frame(p(0.0, 0.0, 10.0), Direction::Z)).build(m)?;
    boolean::difference(m, cylinder, ball)
}
