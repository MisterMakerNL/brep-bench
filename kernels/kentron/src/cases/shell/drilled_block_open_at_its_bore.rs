//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along the
//! line x = 10, y = 10, shelled inward to a wall 1 thick, open at the bore. All six flat faces are
//! walls, moved 1 inward, and the cavity, the box from (1, 1, 1) to (19, 19, 9), runs out to the
//! bore's cylinder, where the rim is two bands each 1 high.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = shell::drilled_block(m, 10.0, 10.0)?;
    // open at the bore, picked by a point on its cylinder
    shell::inward(m, block, &[p(10.0, 7.0, 5.0)], 1.0, Join::Arc)
}
