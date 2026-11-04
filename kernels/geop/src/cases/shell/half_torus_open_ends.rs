//! The half with y ≥ 0 of the solid torus about the z axis whose tube has radius 2 about a
//! middle circle of radius 6, shelled inward to a wall 0.5 thick, open at both flat ends: a
//! curved pipe. geop has no torus: the half torus is turned from its circular section half a
//! turn about the z axis, then shelled by geop's shell, the two ends picked by a point on each.
use crate::shared::hollow::shell;
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(6.0, 0.0), 2.0);
    sketch(b, "tube", zx(), tube)?;
    // A right-handed half turn about +z carries +x to +y: the half with y ≥ 0.
    revolve(b, "half", "tube", axis(FrameAxis::Z), 180.0, Combine::NewBody)?;
    shell(b, "pipe", &[[6.0, 0.0, 0.0], [-6.0, 0.0, 0.0]], 0.5, false)
}
