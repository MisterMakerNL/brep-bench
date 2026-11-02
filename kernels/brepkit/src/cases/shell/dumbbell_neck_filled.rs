//! A dumbbell-shaped plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6,
//! shelled inward to a wall 1.5 thick, open at its top, the wall every point of the plate within
//! 1.5 of its faces other than the top: the neck's moved walls cross and the neck fills, and each
//! pocket bulges towards the neck in arcs of radius 1.5 about its concave corners.
//!
//! Refused: that wall is the shell joined by arcs at the concave edges, with crossing moved faces
//! trimmed away. brepkit's `shell` and `shell_v2` join moved faces only by extending them until
//! they meet (the arc joint, `JointType::Arc` in brepkit-offset, is not implemented), and a sharp
//! shell has no edge whose round would bound the pockets by arcs about the neck's corners.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_: &mut Topology) -> Result<SolidId, Refusal> {
    Err(
        "brepkit's shell joins moved faces only by extending them until they meet (its arc \
         joint is not implemented): it has no shell whose wall is every point within the \
         thickness of the faces, which trims the neck's crossing walls and bounds the pockets by \
         arcs about the neck's corners"
            .into(),
    )
}
