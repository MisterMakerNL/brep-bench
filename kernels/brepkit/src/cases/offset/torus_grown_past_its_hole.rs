//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 4, grown
//! by 3 with brepkit's offset. The torus has no edges, so there is no joint to round or keep
//! sharp: the offset is the torus's one face moved 3 outward, which here passes the axis.
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::make_torus;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let torus = make_torus(topo, 4.0, 2.0, 64)?;
    offset_solid_v2(topo, torus, 3.0)
}
