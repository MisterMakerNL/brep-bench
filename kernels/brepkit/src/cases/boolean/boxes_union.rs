//! Two blocks 10 high standing on the xy plane, the first over x, y 0..10 and the second over
//! x, y 5..15: their union (their tops and their bottoms lie in one plane).
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::block;
use crate::solids::fuse;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let first = block(topo, [0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let second = block(topo, [5.0, 5.0, 0.0], [15.0, 15.0, 10.0])?;
    fuse(topo, first, second)
}
