//! A cube of side 10 at the origin fused with a cylinder of radius 5 and height 10 standing on
//! the xy plane at x = 10, y = 5: half the cylinder lies inside the cube, and its side touches
//! the cube's faces y = 0 and y = 10 along their edges at x = 10.
use brepkit_operations::primitives::make_box;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::upright_cylinder;
use crate::solids::fuse;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let boss = upright_cylinder(topo, 5.0, 10.0, 5.0, 0.0, 10.0)?;
    fuse(topo, cube, boss)
}
