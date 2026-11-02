//! A stepped shaft on the z axis: a cylinder of radius 5 from z = 0 to 10 fused with one of
//! radius 3 from z = 10 to 20 standing on it.
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::upright_cylinder;
use crate::solids::fuse;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let lower = upright_cylinder(topo, 5.0, 0.0, 0.0, 0.0, 10.0)?;
    let upper = upright_cylinder(topo, 3.0, 0.0, 0.0, 10.0, 20.0)?;
    fuse(topo, lower, upper)
}
