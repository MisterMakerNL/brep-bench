//! A ball of radius 5 centred on the origin, with a hole of radius 3 bored right through it along
//! the z axis: the drill a cylinder from z = −10 to z = 10, past the ball at both ends.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(5.0).build(m)?;
    let drill = Cylinder::new(3.0, 20.0).with_frame(frame(p(0.0, 0.0, -10.0), Direction::Z)).build(m)?;
    boolean::difference(m, ball, drill)
}
