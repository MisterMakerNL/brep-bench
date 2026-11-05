//! A sector of a ring about the z axis (radii 8 to 12, z from −1 to 1, polar angle 0° to 60°),
//! grooved by a torus (center (5, 0, 0), radii 2.5 and 2, its axis leaning 25° from z toward +y)
//! turned rigidly about the z axis from −140° to +140°.
//!
//! Refused. cadrs has no sweep of a solid: its `sweep_body` carries points, wires and sheets
//! one kind higher, never a solid. The torus's axis is skew to the turn's, so the groove's wall
//! is its envelope under the turn, at each instant the curve on the torus where its velocity is
//! tangent to it, none of its own circles, which the kernel does not compute; fitted to points
//! here, it would be geometry made outside the kernel.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no sweep of a solid (its sweep_body carries points, wires and sheets, never a \
         solid): the groove is the envelope of a torus turned about an axis skew to its own, \
         which it cannot build",
    ))
}
