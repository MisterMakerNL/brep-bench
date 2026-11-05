//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! ground by a torus (radii 6 and 2, centered (11.5, 0, 0), its axis leaning by the helix angle)
//! carried along the screw about the z axis that rises 30 a turn, from a turn of −108° to +348°.
//!
//! Refused. cadrs has no sweep of a solid: its `sweep_body` carries points, wires and sheets
//! one kind higher, only straight along a vector or about one axis, not along a screw. The
//! wheel's sweep under the screw is bounded by its envelope, at each instant the curve on the
//! torus where its velocity is tangent to it, which is none of the torus's circles and which the
//! kernel does not compute; fitted to points here, it would be geometry made outside the kernel.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no sweep of a solid, and sweeps points, wires and sheets only straight or \
         about one axis: the groove is the envelope of the leaning torus under a screw motion, \
         which it cannot build",
    ))
}
