//! A napkin ring: a ball of radius 5 about the origin, bored through along the z axis by a
//! cylinder of radius 3 running past the ball at both ends.
use brepkit_operations::primitives::make_sphere;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::upright_cylinder;
use crate::solids::cut;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let ball = make_sphere(topo, 5.0, 64)?;
    let drill = upright_cylinder(topo, 3.0, 0.0, 0.0, -6.0, 6.0)?;
    cut(topo, ball, drill)
}
