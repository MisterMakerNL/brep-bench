//! A cylinder of radius 5 on the z axis from z = 0 to 10, less a ball of radius 5 about the middle
//! of its top face, whose equator is the top face's rim.
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::ball;
use crate::solids::cut;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let drum = make_cylinder(topo, 5.0, 10.0)?;
    let dish = ball(topo, 5.0, (0.0, 0.0, 10.0))?;
    Ok(cut(topo, drum, dish)?)
}
