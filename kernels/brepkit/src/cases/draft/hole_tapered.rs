//! A block from (−15, −15, 0) to (15, 15, 20) bored through on the z axis by a hole of radius 6,
//! the hole's wall drafted by 3° with the pull up +z about the neutral plane z = 0, into a cone
//! widening toward the top.
//!
//! Refused: brepkit's draft (`draft::draft`) tilts planar faces only, turning each plane about
//! its neutral line and re-solving the vertices it moves; it declines a face that is not a plane
//! and a vertex where a drafted face meets one that is not.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_: &mut Topology) -> Result<SolidId, Refusal> {
    Err("brepkit's draft tilts planar faces only, and the hole's wall is a cylinder".into())
}
