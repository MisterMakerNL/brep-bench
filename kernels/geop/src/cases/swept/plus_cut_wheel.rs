//! A wheel cut by a plus-shaped tool turning in mesh with it, as a gear shaper cuts: the tool
//! turning about the z axis while the wheel turns two thirds as fast the other way about its own.
//!
//! geop sweeps a profile along a path, turning it with the path; it has no sweep of a solid
//! through a motion, and what the tool passes through, seen from the wheel, is bounded by the
//! envelope of its faces as both turn — a curve that would have to be computed here, outside
//! geop.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no sweep of a solid through a motion: the tool's sweep as both it and the wheel \
         turn is bounded by an envelope geop cannot build"
        .into())
}
