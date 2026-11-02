//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick with no opening: brepkit's
//! `shell` with no open face moves every face 1 inward to make the cavity, the box from (1, 1, 1)
//! to (9, 7, 5), a second shell of the solid facing the void.
use brepkit_operations::primitives::make_box;
use brepkit_operations::shell_op::shell;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let block = make_box(topo, 10.0, 8.0, 6.0)?;
    shell(topo, block, 1.0, &[])
}
