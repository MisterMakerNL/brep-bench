//! The box from the origin to (10, 8, 6) shelled inward to a wall 4 thick, open at its top face,
//! z = 6. The walls at y = 0 and y = 8, moved 4 inward, meet in the plane y = 4: the cavity is
//! flat, nothing is hollowed, and the result is the box itself, its six faces as they were.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // open at the top, z = 6
    shell::inward(m, block, &[p(5.0, 4.0, 6.0)], 4.0, Join::Arc)
}
