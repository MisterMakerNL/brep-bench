//! The L-shaped plate, the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
//! extruded 8 along +z, shrunk by 1: every point at least 1 from its outside. Its convex edges stay
//! sharp; the concave edge along (5, 5) becomes a concave quarter cylinder of radius 1.
use crate::shared::offset::l_plate;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = l_plate(m)?;
    offset::shape(m, plate, -1.0, Join::Arc)
}
