//! A torus about the z axis whose hole is nearly closed: brepkit's torus with a middle circle of
//! radius 5 and a tube of radius 4.99. The segment count only sets how finely it is meshed.
use brepkit_operations::primitives::make_torus;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    make_torus(topo, 5.0, 4.99, 64)
}
