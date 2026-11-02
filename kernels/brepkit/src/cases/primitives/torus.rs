//! A torus about the z axis, centred on the origin: a tube of radius 2 round a circle of radius
//! 10. brepkit's torus lies there already; the segment count only sets how finely it is meshed.
use brepkit_operations::primitives::make_torus;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    make_torus(topo, 10.0, 2.0, 64)
}
