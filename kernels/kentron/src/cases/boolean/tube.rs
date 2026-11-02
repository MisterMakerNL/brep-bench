//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, with a hole of radius 3 bored along
//! the axis: the drill a cylinder from z = −1 to z = 11, past both ends.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 10.0).build(m)?;
    let drill = Cylinder::new(3.0, 12.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    boolean::difference(m, rod, drill)
}
