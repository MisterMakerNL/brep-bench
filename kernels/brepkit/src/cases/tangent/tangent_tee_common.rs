//! Two cylinders crossing at right angles: one of radius 5 along the x axis from x = −15 to 15,
//! one of radius 3 parallel to y from y = −15 to 15 on the line x = 0, z = 2, so that the tops of
//! both are at z = 5 and touch at (0, 0, 5). Their common part.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::common;
use crate::solids::contact::{cylinder_x, cylinder_y};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let first = cylinder_x(topo, 5.0, (0.0, 0.0), -15.0, 15.0)?;
    let second = cylinder_y(topo, 3.0, (0.0, 2.0), -15.0, 15.0)?;
    Ok(common(topo, first, second)?)
}
