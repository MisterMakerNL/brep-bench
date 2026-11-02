//! A block from the origin to (40, 30, 15) less a pocket from (8, 8, 5) to (32, 22, 15) whose
//! vertical edges are rounded to radius 4, the pocket's eight walls drafted by 5° with the pull up
//! +z about the neutral plane z = 5, the planes turning about their bottom edges and the quarter
//! cylinders becoming quarter cones.
//!
//! Refused: brepkit's draft (`draft::draft`) tilts planar faces only, turning each plane about
//! its neutral line and re-solving the vertices it moves; it declines a face that is not a plane
//! and a vertex where a drafted face meets one that is not.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_: &mut Topology) -> Result<SolidId, Refusal> {
    Err("brepkit's draft tilts planar faces only, and four of the pocket's eight walls are quarter cylinders (its draft also declines a vertex where a drafted plane meets one)".into())
}
