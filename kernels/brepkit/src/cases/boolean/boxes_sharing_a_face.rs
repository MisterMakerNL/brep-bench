//! Two cubes of side 10 side by side, at the origin and at (10, 0, 0), fused into one solid: the
//! face they share at x = 10 is gone from the result.
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::block;
use crate::solids::fuse;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let first = block(topo, [0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let second = block(topo, [10.0, 0.0, 0.0], [20.0, 10.0, 10.0])?;
    fuse(topo, first, second)
}
