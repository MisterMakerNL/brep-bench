//! Two balls of radius 5, about the origin and about (9.999999, 0, 0), overlapping only 10⁻⁶ deep:
//! the lens common to both.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::common;
use crate::solids::contact::ball;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let left = ball(topo, 5.0, (0.0, 0.0, 0.0))?;
    let right = ball(topo, 5.0, (9.999999, 0.0, 0.0))?;
    Ok(common(topo, left, right)?)
}
