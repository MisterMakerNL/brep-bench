//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along the
//! line x = 15.5, y = 10, shelled inward to a wall 1 thick, open at its top: the bore moved 1
//! inward, the cylinder of radius 4, reaches past the inside wall x = 19, so the tube round the
//! bore merges with that wall.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{drilled_block, face_through};
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = drilled_block(topo, 15.5, 10.0)?;
    let open = face_through(topo, block, p(2.0, 2.0, 10.0))?;
    Ok(shell(topo, block, 1.0, &[open])?)
}
