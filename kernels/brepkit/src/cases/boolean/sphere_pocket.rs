//! A block from (−10, −10, −10) to (10, 10, 0) with a ball of radius 5 about the origin cut from
//! it, leaving a hemispherical pocket in its top face.
use brepkit_operations::primitives::make_sphere;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::block;
use crate::solids::cut;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let blank = block(topo, [-10.0, -10.0, -10.0], [10.0, 10.0, 0.0])?;
    let ball = make_sphere(topo, 5.0, 64)?;
    cut(topo, blank, ball)
}
