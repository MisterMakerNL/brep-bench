//! A cube of side 10 between the origin and (10, 10, 10), with a hole of radius 3 drilled straight
//! through it along z at x = 5, y = 5: the drill a cylinder from z = −1 to z = 11, past both faces.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let drill = Cylinder::new(3.0, 12.0).with_frame(frame(p(5.0, 5.0, -1.0), Direction::Z)).build(m)?;
    boolean::difference(m, cube, drill)
}
