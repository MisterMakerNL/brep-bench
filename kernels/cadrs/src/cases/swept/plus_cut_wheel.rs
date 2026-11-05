//! A wheel, the disk of radius 40 about the vertical line through (−50, 0) from z = −8 to −2,
//! cut by a plus (arms 30 long and 8 wide along ±x and ±y, from z = −10 to 0) turning about the
//! z axis by t while the wheel turns by −2t/3 about its own axis, over t from 0° to 540°.
//!
//! Refused. cadrs has no sweep of a solid: its `sweep_body` carries points, wires and sheets
//! one kind higher, only straight along a vector or about one axis. Seen from the wheel the plus
//! rolls in its plane, so the cut is bounded by the envelopes of its sides under that roll and
//! the trochoids its corners trace, curves the kernel has no form of and does not compute;
//! fitted to points of the envelope here, they would be geometry made outside the kernel.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no sweep of a solid, and sweeps points, wires and sheets only straight or \
         about one axis: the wheel's flanks are the envelope of the plus's sides as it rolls in \
         mesh with the wheel and the trochoids its corners trace, which it cannot build",
    ))
}
