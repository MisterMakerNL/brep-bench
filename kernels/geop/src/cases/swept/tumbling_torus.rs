//! The solid a torus fills as it turns about one axis while watched from a frame turning about
//! another.
//!
//! geop sweeps a profile along a path; it has no sweep of a solid through a motion, and this one
//! is bounded by an envelope of no simpler kind, which would have to be computed here, outside
//! geop.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no sweep of a solid through a motion: the tumbling torus's sweep is bounded by \
         an envelope geop cannot build"
        .into())
}
