//! The box from the origin to (10, 8, 6) shelled inward to a wall 3.99 thick, open at its top
//! face, z = 6: the walls at y = 0 and y = 8 come within 0.02 of each other, leaving a slot from
//! (3.99, 3.99, 3.99) to (6.01, 4.01, 6) open at the top.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // open at the top, z = 6
    shell::inward(m, block, &[p(5.0, 4.0, 6.0)], 3.99, Join::Arc)
}
