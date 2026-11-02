//! A cube of side 10 at the origin, drilled through along z by a cylinder of radius 3 on the
//! line x = 5, y = 5, the cylinder running past both faces (z = −1 to 11).
use brepkit_operations::primitives::make_box;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::upright_cylinder;
use crate::solids::cut;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let drill = upright_cylinder(topo, 3.0, 5.0, 5.0, -1.0, 11.0)?;
    cut(topo, cube, drill)
}
