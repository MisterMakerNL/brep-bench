//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at its top, z =
//! 6, and its bottom, z = 0: a rectangular tube, the box less the box from (1, 1, 0) to (9, 7, 6).
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // open at the top, z = 6, and the bottom, z = 0
    shell::inward(m, block, &[p(5.0, 4.0, 6.0), p(5.0, 4.0, 0.0)], 1.0, Join::Arc)
}
