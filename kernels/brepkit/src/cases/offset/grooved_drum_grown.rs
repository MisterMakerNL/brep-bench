//! A drum, the cylinder of radius 10 on the z axis from z = 0 to 10, with a groove round its side
//! from z = 4.2 to 5.8 cut to radius 7, grown by 1 with brepkit's rounded offset, whose joints
//! roll a ball of radius 1 round its rims.
use brepkit_operations::offset_v2::offset_solid_arc_v2;
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{cut, moved};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let drum = make_cylinder(topo, 10.0, 10.0)?;
    // the groove: a ring from radius 7 out past the drum's side
    let outer = make_cylinder(topo, 11.0, 1.6)?;
    let outer = moved(topo, outer, 0.0, 0.0, 4.2)?;
    let core = make_cylinder(topo, 7.0, 3.0)?;
    let core = moved(topo, core, 0.0, 0.0, 3.5)?;
    let groove = cut(topo, outer, core)?;
    let grooved = cut(topo, drum, groove)?;
    Ok(offset_solid_arc_v2(topo, grooved, 1.0)?)
}
