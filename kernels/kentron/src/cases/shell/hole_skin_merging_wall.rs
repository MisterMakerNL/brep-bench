//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along the
//! line x = 15.5, y = 10, shelled inward to a wall 1 thick, open at its top face, z = 10. The bore,
//! moved 1 into the material, is the cylinder of radius 4, which runs past the inside wall x = 19:
//! the tube round the bore merges with that wall.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = shell::drilled_block(m, 15.5, 10.0)?;
    // open at the top, z = 10, picked by a point away from the hole
    shell::inward(m, block, &[p(1.0, 1.0, 10.0)], 1.0, Join::Arc)
}
