//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical V groove
//! ground by a wheel with a sharp 60° V rim (its axis through (14, 0, 0) leaning by the helix
//! angle) carried along the screw about the z axis that rises 30 a turn, from a turn of −144° to
//! +384°.
//!
//! Refused. cadrs has no sweep of a solid: its `sweep_body` carries points, wires and sheets
//! one kind higher, only straight along a vector or about one axis, not along a screw. The
//! wheel's flanks are cones whose sweep under the screw is bounded by their envelope, which the
//! kernel does not compute; fitted to points here, it would be geometry made outside the kernel.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no sweep of a solid, and sweeps points, wires and sheets only straight or \
         about one axis: the groove is the envelope of the V wheel's flanks under a screw \
         motion, which it cannot build",
    ))
}
