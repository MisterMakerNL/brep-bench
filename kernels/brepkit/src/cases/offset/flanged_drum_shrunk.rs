//! A drum, the cylinder of radius 8 on the z axis from z = 0 to 6, with a flange round its side
//! from radius 8 to 11 and from z = 2.2 to 3.8, shrunk by 1 with brepkit's rounded offset, whose
//! joints roll a ball of radius 1 along the flange's inside edges.
use brepkit_operations::offset_v2::offset_solid_arc_v2;
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{fuse, moved};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let drum = make_cylinder(topo, 8.0, 6.0)?;
    let flange = make_cylinder(topo, 11.0, 1.6)?;
    let flange = moved(topo, flange, 0.0, 0.0, 2.2)?;
    let flanged = fuse(topo, drum, flange)?;
    Ok(offset_solid_arc_v2(topo, flanged, -1.0)?)
}
