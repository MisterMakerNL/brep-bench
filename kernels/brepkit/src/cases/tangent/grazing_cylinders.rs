//! Two cylinders of radius 5 and length 30 crossing at right angles, the first along the x axis
//! and the second parallel to y on the line x = 0, z = 9.99, so that they overlap only 0.01 deep;
//! their common part.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::common;
use crate::solids::contact::{cylinder_x, cylinder_y};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let first = cylinder_x(topo, 5.0, (0.0, 0.0), -15.0, 15.0)?;
    let second = cylinder_y(topo, 5.0, (0.0, 9.99), -15.0, 15.0)?;
    Ok(common(topo, first, second)?)
}
