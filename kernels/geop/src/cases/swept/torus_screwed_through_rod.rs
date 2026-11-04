//! A rod grooved by a torus carried along a screw about the z axis, its axis leaning by the helix
//! angle, as a thread is ground.
//!
//! geop sweeps a profile along a path; it has no sweep of a solid, and what the torus passes
//! through is bounded by the envelope of its surface along the screw — a surface of no simpler
//! kind, which would have to be computed here, outside geop.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no sweep of a solid: what a torus screwed along the rod passes through is \
         bounded by the envelope of its surface, which geop cannot build"
        .into())
}
