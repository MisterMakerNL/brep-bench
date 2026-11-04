//! A curved block grooved by a torus turned past it about the z axis, its axis skew to the turn.
//!
//! geop sweeps a profile along a path; it has no sweep of a solid, and what the torus passes
//! through is bounded by the envelope of its surface as it turns — none of its own circles, since
//! its axis is skew to the turn's — which would have to be computed here, outside geop.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no sweep of a solid: what a torus turned about an axis skew to its own passes \
         through is bounded by the envelope of its surface, which geop cannot build"
        .into())
}
