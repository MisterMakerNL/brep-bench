//! The box from the origin to (30, 20, 10) with its four vertical edges rounded to radius 4, all
//! eight sides drafted by 5° with the pull up +z about the neutral plane z = 0, the planes turning
//! about their bottom edges and the quarter cylinders becoming quarter cones.
//!
//! Refused: brepkit's draft (`draft::draft`) tilts planar faces only, turning each plane about
//! its neutral line and re-solving the vertices it moves; it declines a face that is not a plane
//! and a vertex where a drafted face meets one that is not.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_: &mut Topology) -> Result<SolidId, Refusal> {
    Err("brepkit's draft tilts planar faces only, and four of the box's eight sides are quarter cylinders (its draft also declines a vertex where a drafted plane meets one)".into())
}
