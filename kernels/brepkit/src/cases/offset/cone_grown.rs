//! A cone with a base of radius 3 at z = 0 and its apex at (0, 0, 4), grown by 1 with brepkit's
//! rounded offset, whose joints roll a ball of radius 1 round the rim and over the apex.
use brepkit_operations::offset_v2::offset_solid_arc_v2;
use brepkit_operations::primitives::make_cone;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let cone = make_cone(topo, 3.0, 0.0, 4.0)?;
    offset_solid_arc_v2(topo, cone, 1.0)
}
