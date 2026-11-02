//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along the
//! line x = 10, y = 10, shelled inward to a wall 1 thick, open at the bore: the six flat faces are
//! moved 1 inward, and the cavity runs out to the bore's cylinder, the rim two bands on it.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{drilled_block, face_through};
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = drilled_block(topo, 10.0, 10.0)?;
    let open = face_through(topo, block, p(10.0, 7.0, 5.0))?;
    Ok(shell(topo, block, 1.0, &[open])?)
}
