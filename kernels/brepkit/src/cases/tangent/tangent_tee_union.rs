//! A tee: a main cylinder of radius 5 along the x axis from x = −15 to 15, united with a branch of
//! radius 3 parallel to y from y = −15 to 15 on the line x = 0, z = 2, whose top touches the
//! main's crown at (0, 0, 5).
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::{cylinder_x, cylinder_y};
use crate::solids::fuse;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let main = cylinder_x(topo, 5.0, (0.0, 0.0), -15.0, 15.0)?;
    let branch = cylinder_y(topo, 3.0, (0.0, 2.0), -15.0, 15.0)?;
    Ok(fuse(topo, main, branch)?)
}
