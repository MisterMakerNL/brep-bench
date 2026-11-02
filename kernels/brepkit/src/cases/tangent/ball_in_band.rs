//! A ball of radius 5 about the origin united with a cylinder of the same radius on the z axis,
//! from z = −3 to z = 3, whose wall touches the ball along its equator.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::{ball, cylinder_z};
use crate::solids::fuse;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let sphere = ball(topo, 5.0, (0.0, 0.0, 0.0))?;
    let band = cylinder_z(topo, 5.0, (0.0, 0.0), -3.0, 3.0)?;
    Ok(fuse(topo, sphere, band)?)
}
