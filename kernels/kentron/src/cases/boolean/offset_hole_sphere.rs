//! A ball of radius 20 centred on the origin, with a round hole of radius 6 bored right through it
//! parallel to the y axis, on the line x = 5, z = 0: it enters and leaves through the sphere and
//! never breaks out of its side. The drill a cylinder from y = −25 to y = 25, past the ball.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(20.0).build(m)?;
    let drill = Cylinder::new(6.0, 50.0).with_frame(frame(p(5.0, -25.0, 0.0), Direction::Y)).build(m)?;
    boolean::difference(m, ball, drill)
}
