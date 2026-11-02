//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at two faces that
//! share an edge, its top, z = 6, and its front, y = 0: the box less the box from (1, 0, 1) to
//! (9, 7, 6), a bottom, a back and two sides, each 1 thick.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // open at the top, z = 6, and the front, y = 0, which share the edge y = 0, z = 6
    shell::inward(m, block, &[p(5.0, 4.0, 6.0), p(5.0, 0.0, 3.0)], 1.0, Join::Arc)
}
