//! A gear blank, the disc of radius 22 about the y axis from y = 0 to y = 6, with one tooth space
//! cut by a rack tooth (module 2, 20° pressure angle) rolled round it: the blank turning about
//! the y axis while the rack slides along its pitch line, 2π·20 a turn, over a third of a turn.
//! Everything the tooth passes through, cut from the blank.
//!
//! Refused. Seen from the blank the tooth turns and slides at once, a rolling motion about no one
//! fixed axis; solidcore sweeps a whole body only along a line or about one fixed axis
//! (`SmApiNonManifoldSweep`, `SmApiNonManifoldRotationalSweep`). The space's flanks are the
//! envelopes of the tooth's flanks (involutes) and its root the curve its tip corners trace,
//! geometry that would be computed outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("seen from the blank the rack tooth rolls, turning and sliding at once; solidcore sweeps a \
         whole body only along a line or about one fixed axis, and the space's flanks are \
         envelopes of the tooth's (involutes), which would be computed outside the kernel"
        .into())
}
