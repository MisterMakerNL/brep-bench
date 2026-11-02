//! The L-shaped plate, the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
//! extruded 8 along +z, grown by 1: every point within 1 of it. Its convex edges become quarter
//! cylinders and its convex corners eighth balls; the concave edge along (5, 5) stays sharp.
use crate::shared::offset::l_plate;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = l_plate(m)?;
    offset::shape(m, plate, 1.0, Join::Arc)
}
