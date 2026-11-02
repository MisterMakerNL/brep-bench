//! A ball of radius 5 about the origin hollowed to a wall 1 thick with no opening: brepkit's
//! `shell` with no open face moves the sphere 1 inward, the sphere of radius 4 bounding the
//! cavity.
use brepkit_operations::primitives::make_sphere;
use brepkit_operations::shell_op::shell;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let ball = make_sphere(topo, 5.0, 64)?;
    shell(topo, ball, 1.0, &[])
}
