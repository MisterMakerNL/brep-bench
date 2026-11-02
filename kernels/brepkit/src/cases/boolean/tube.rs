//! A tube: a cylinder of radius 5 on the z axis from z = 0 to 10, bored along its axis by a
//! cylinder of radius 3 running past both ends (z = −1 to 11).
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::upright_cylinder;
use crate::solids::cut;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let rod = make_cylinder(topo, 5.0, 10.0)?;
    let drill = upright_cylinder(topo, 3.0, 0.0, 0.0, -1.0, 11.0)?;
    cut(topo, rod, drill)
}
