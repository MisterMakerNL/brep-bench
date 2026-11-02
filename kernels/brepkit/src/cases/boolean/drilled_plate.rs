//! A plate from the origin to (100, 20, 10), drilled nine times along z: holes of radius 2 at
//! y = 10 and x = 10, 20, … 90, each its own cut by a cylinder running past both faces.
use brepkit_operations::primitives::make_box;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::upright_cylinder;
use crate::solids::cut;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let mut plate = make_box(topo, 100.0, 20.0, 10.0)?;
    for i in 1..=9 {
        let drill = upright_cylinder(topo, 2.0, 10.0 * f64::from(i), 10.0, -1.0, 11.0)?;
        plate = cut(topo, plate, drill)?;
    }
    Ok(plate)
}
