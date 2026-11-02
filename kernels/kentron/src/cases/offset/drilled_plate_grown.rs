//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 1: every point within 1 of it. The hole narrows to radius 2 and flares to
//! radius 3 at the grown faces, its rims saddles. The drill runs 1 past both faces.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 4.0)).build(m)?;
    let drill = Cylinder::new(3.0, 6.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let drilled = boolean::difference(m, plate, drill)?;
    offset::shape(m, drilled, 1.0, Join::Arc)
}
