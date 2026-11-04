//! A quarter of a washer: the rectangle with x from 6 to 10 and z from 0 to 4, sketched on the
//! xz plane, revolved 90° about the z axis, right-handed (from +x towards +y).
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    section.rectangle(zx_at(6.0, 0.0), zx_at(10.0, 4.0));
    sketch(b, "section", zx(), section)?;
    revolve(b, "sector", "section", axis(FrameAxis::Z), 90.0, Combine::NewBody)
}
