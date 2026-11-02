//! A ball of radius 5 about the origin, split by the xy plane with brepkit's `split`; the half
//! above it is kept, everything below cut away.
use brepkit_operations::primitives::make_sphere;
use brepkit_operations::split::split;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{p, v};

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let ball = make_sphere(topo, 5.0, 64)?;
    Ok(split(topo, ball, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0))?.positive)
}
