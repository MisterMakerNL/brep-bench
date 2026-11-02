//! A plate from (−20, −20, 0) to (20, 20, 5) with a boss of radius 8 on the z axis from z = 5
//! to 17, the boss's side drafted by 4° with the pull up +z about the neutral plane z = 5, into a
//! cone.
//!
//! Refused: brepkit's draft (`draft::draft`) tilts planar faces only, turning each plane about
//! its neutral line and re-solving the vertices it moves; it declines a face that is not a plane
//! and a vertex where a drafted face meets one that is not.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_: &mut Topology) -> Result<SolidId, Refusal> {
    Err("brepkit's draft tilts planar faces only, and the boss's side is a cylinder".into())
}
