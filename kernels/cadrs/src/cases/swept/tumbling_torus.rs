//! The solid a torus (center (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by t/4
//! about the vertical line through (2, 0, 0), seen from a frame turning by t about the line
//! through (0, 0.5, 0.5) along x, as t runs from −75° to 75°.
//!
//! Refused. cadrs has no sweep of a solid: its `sweep_body` carries points, wires and sheets
//! one kind higher, only straight along a vector or about one axis. The two turns about skew
//! axes are neither, and keep no line of the torus fixed, so its sweep's boundary is the
//! envelope of the torus under the motion, which the kernel does not compute; fitted to points
//! here, it would be geometry made outside the kernel.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no sweep of a solid, and sweeps points, wires and sheets only straight or \
         about one axis: the tumbling torus's boundary is its envelope under two turns about \
         skew axes, which it cannot build",
    ))
}
