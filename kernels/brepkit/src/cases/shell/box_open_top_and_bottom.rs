//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at its top, z = 6,
//! and its bottom, z = 0: a rectangular tube with a rim at each end.
use brepkit_operations::primitives::make_box;
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::faces_through;
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 10.0, 8.0, 6.0)?;
    // the openings, each picked by a point on it
    let open = faces_through(topo, block, &[p(5.0, 4.0, 6.0), p(5.0, 4.0, 0.0)])?;
    Ok(shell(topo, block, 1.0, &open)?)
}
