//! A cylinder of radius 5 on the z axis from z = 0 to 10: brepkit's cylinder stands there
//! already, its base on the origin.
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    make_cylinder(topo, 5.0, 10.0)
}
