//! A torus about the z axis, its tube of radius 3 about a circle of radius 10 in the xy plane,
//! united with a drum of radius 13 on the z axis from z = −1 to 1, whose wall touches the torus
//! along its outer equator. brepkit's torus lies there already.
use brepkit_operations::primitives::make_torus;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::cylinder_z;
use crate::solids::fuse;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let ring = make_torus(topo, 10.0, 3.0, 64)?;
    let drum = cylinder_z(topo, 13.0, (0.0, 0.0), -1.0, 1.0)?;
    Ok(fuse(topo, ring, drum)?)
}
