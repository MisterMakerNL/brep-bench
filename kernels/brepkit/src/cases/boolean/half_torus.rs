//! A torus about the z axis (tube centre circle radius 10, tube radius 2), split by the plane
//! x = 0 with brepkit's `split`; the half on the +x side is kept, the half where x < 0 cut away.
use brepkit_operations::primitives::make_torus;
use brepkit_operations::split::split;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{p, v};

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let torus = make_torus(topo, 10.0, 2.0, 64)?;
    Ok(split(topo, torus, p(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0))?.positive)
}
