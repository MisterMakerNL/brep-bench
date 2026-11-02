//! A truncated cone on the z axis, radius 5 at z = 0 and radius 2 at z = 6: brepkit's cone takes
//! its bottom and top radii and stands on the origin already.
use brepkit_operations::primitives::make_cone;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    make_cone(topo, 5.0, 2.0, 6.0)
}
