//! Two balls of radius 5, one about the origin and one about (6, 0, 0): their union.
use brepkit_operations::primitives::make_sphere;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{fuse, moved};

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let first = make_sphere(topo, 5.0, 64)?;
    let second = make_sphere(topo, 5.0, 64)?;
    let second = moved(topo, second, 6.0, 0.0, 0.0)?;
    fuse(topo, first, second)
}
