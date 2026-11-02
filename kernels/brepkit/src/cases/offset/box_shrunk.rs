//! The box from the origin to (10, 8, 6) shrunk by 1: every face moved 1 inward by brepkit's
//! offset, whose default joint extends the moved faces until they meet, so the edges stay sharp.
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::make_box;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let block = make_box(topo, 10.0, 8.0, 6.0)?;
    offset_solid_v2(topo, block, -1.0)
}
