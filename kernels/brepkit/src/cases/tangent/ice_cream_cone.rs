//! A ball of radius 15 about the origin united with a cone on the z axis, its apex at
//! (0, 0, −25) and its base the disc of radius 12 at z = −9, whose side is tangent to the sphere
//! along the rim of its base. brepkit's cone with a bottom radius of 0, raised to the apex.
use brepkit_operations::primitives::make_cone;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::ball;
use crate::solids::{fuse, moved};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let scoop = ball(topo, 15.0, (0.0, 0.0, 0.0))?;
    let cone = make_cone(topo, 0.0, 12.0, 16.0)?;
    let cone = moved(topo, cone, 0.0, 0.0, -25.0)?;
    Ok(fuse(topo, scoop, cone)?)
}
