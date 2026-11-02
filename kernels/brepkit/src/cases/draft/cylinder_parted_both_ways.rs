//! A cylinder of radius 10 on the z axis from z = 0 to 20, its side split at the parting line
//! z = 10 and each half drafted by 3° away from it, into two cones meeting there.
//!
//! Refused: brepkit's draft (`draft::draft`) tilts planar faces only, turning each plane about
//! its neutral line and re-solving the vertices it moves; it declines a face that is not a plane
//! and a vertex where a drafted face meets one that is not.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_: &mut Topology) -> Result<SolidId, Refusal> {
    Err("brepkit's draft tilts planar faces only, and the cylinder's side, above and below its parting line, is cylindrical".into())
}
