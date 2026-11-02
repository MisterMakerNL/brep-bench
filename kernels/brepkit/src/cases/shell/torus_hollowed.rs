//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 6, hollowed
//! to a wall 0.5 thick with no opening: brepkit's `shell` with no open face moves the torus 0.5
//! inward, the torus of tube radius 1.5 bounding the cavity.
use brepkit_operations::primitives::make_torus;
use brepkit_operations::shell_op::shell;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let torus = make_torus(topo, 6.0, 2.0, 64)?;
    shell(topo, torus, 0.5, &[])
}
