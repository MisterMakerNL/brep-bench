//! The solid a torus (centre (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by t/4
//! about the vertical line through (2, 0, 0), seen from a frame turning by t about the line
//! through (0, 0.5, 0.5) along x, as t runs from −75° to 75°.
//!
//! Refused. solidcore has no sweep of a solid, and its sweeps carry curves along lines, arcs,
//! paths and screws. The two turns about skew axes are none of those motions and keep no line
//! of the torus fixed, so its sweep's boundary is the envelope of the torus under the motion, at
//! each instant the curve on it where its velocity is tangent to it, which the kernel does not
//! compute; fitted to points here, it would be geometry made outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore has no sweep of a solid, and its sweeps carry curves along lines, arcs, paths \
         and screws: the tumbling torus's boundary is its envelope under two turns about skew \
         axes, which it cannot build"
        .into())
}
