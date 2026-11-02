//! A cube of side 10 at the origin united with its inscribed ball, of radius 5 about (5, 5, 5).
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::ball;
use crate::solids::fuse;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let inscribed = ball(topo, 5.0, (5.0, 5.0, 5.0))?;
    Ok(fuse(topo, cube, inscribed)?)
}
