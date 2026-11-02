//! A flat lug, the plate from (0, −5, 0) to (80, 5, 40), with a round pin hole of radius 8 straight
//! through its thickness along y on the line x = 60, z = 20: the drill a cylinder from y = −6 to
//! y = 6, past both faces.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(80.0, 5.0, 40.0)).build(m)?;
    let drill = Cylinder::new(8.0, 12.0).with_frame(frame(p(60.0, -6.0, 20.0), Direction::Y)).build(m)?;
    boolean::difference(m, plate, drill)
}
