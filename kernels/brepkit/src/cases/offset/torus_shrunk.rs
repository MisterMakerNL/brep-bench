//! A solid torus about the z axis, its tube of radius 3 about a middle circle of radius 10,
//! shrunk by 1.5 with brepkit's offset: its one face moved 1.5 inward, to the torus of tube
//! radius 1.5. The torus has no edges, so there is no joint to round or keep sharp.
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::make_torus;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let torus = make_torus(topo, 10.0, 3.0, 64)?;
    offset_solid_v2(topo, torus, -1.5)
}
