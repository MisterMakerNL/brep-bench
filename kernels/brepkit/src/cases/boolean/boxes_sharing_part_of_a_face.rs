//! Two cubes of side 10 fused into one solid: the first at the origin, the second between
//! (5, 0, 10) and (15, 10, 20), standing on the first and overhanging half of its top.
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::block;
use crate::solids::fuse;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let first = block(topo, [0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let second = block(topo, [5.0, 0.0, 10.0], [15.0, 10.0, 20.0])?;
    fuse(topo, first, second)
}
