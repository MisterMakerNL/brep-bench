//! A rod with a helical V groove ground by a sharp-rimmed wheel carried along a screw about the z
//! axis, its axis leaning by the helix angle.
//!
//! geop sweeps a profile along a path; it has no sweep of a solid, and what the wheel passes
//! through is bounded by the envelope of its conical flanks along the screw, which would have to
//! be computed here, outside geop.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no sweep of a solid: what a V wheel screwed along the rod passes through is \
         bounded by the envelope of its flanks, which geop cannot build"
        .into())
}
