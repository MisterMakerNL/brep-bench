//! A cylinder of radius 5 on the z axis from z = 0 to 10, with a cylinder of radius 3 on the
//! same axis and exactly the same span cut from it: the cutter's ends lie in the first's ends.
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::cut;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let rod = make_cylinder(topo, 5.0, 10.0)?;
    let core = make_cylinder(topo, 3.0, 10.0)?;
    cut(topo, rod, core)
}
