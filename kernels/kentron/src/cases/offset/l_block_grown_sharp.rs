//! The L-shaped plate, the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
//! extruded 8 along +z, grown by 1 with its edges kept sharp: each face moved 1 outward and the
//! moved faces extended until they meet.
use crate::shared::offset::l_plate;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = l_plate(m)?;
    offset::shape(m, plate, 1.0, Join::Intersection)
}
