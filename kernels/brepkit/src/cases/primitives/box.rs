//! A block 10 × 20 × 30 with one corner at the origin: brepkit's box is anchored there already.
use brepkit_operations::primitives::make_box;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    make_box(topo, 10.0, 20.0, 30.0)
}
