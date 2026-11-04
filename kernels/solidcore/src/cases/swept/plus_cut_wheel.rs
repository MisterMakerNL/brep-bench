//! A wheel, the disc of radius 40 about the vertical line through (-50, 0) from z = -8 to -2, cut
//! by a plus-shaped tool turning in mesh with it: the tool turns by t about the z axis while the
//! wheel turns by -2t/3 about its own axis, for t from 0° to 540°. Everything the tool passes
//! through, as seen from the wheel, cut from it.
//!
//! Refused. Seen from the wheel the tool turns about the z axis while that axis itself goes round
//! the wheel's: a motion about no one fixed axis. solidcore sweeps a whole body only along a line
//! or about one fixed axis (`SmApiNonManifoldSweep`, `SmApiNonManifoldRotationalSweep`), and the
//! sweep's walls are envelopes of the plus's sides, which would be geometry computed outside the
//! kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("seen from the wheel the tool turns about an axis that itself goes round the wheel's; \
         solidcore sweeps a whole body only along a line or about one fixed axis, and the cut's \
         walls are envelopes of the plus's sides, which would be computed outside the kernel"
        .into())
}
