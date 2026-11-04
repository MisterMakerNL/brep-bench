//! The solid a torus fills as it tumbles: the torus centred on (3, 0, 2), axis along z, radii 1
//! and 0.5, turned by t/4 about the vertical line through (2, 0, 0) and seen from a frame turned
//! by t about the line through (0, 0.5, 0.5) along x, for t from -75° to 75°.
//!
//! Refused. solidcore sweeps a whole body only along a line or about one fixed axis
//! (`SmApiNonManifoldSweep`, `SmApiNonManifoldRotationalSweep`); this motion turns about two axes
//! at once, and its sweep is bounded by an envelope of no simpler kind, which would be geometry
//! computed outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore sweeps a whole body only along a line or about one fixed axis; the torus's \
         motion turns about two axes at once, and its sweep's envelope would be computed outside \
         the kernel"
        .into())
}
