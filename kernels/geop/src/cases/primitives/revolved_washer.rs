//! A washer: the rectangle with x from 6 to 10 and z from 0 to 4, sketched on the xz plane,
//! revolved a full turn about the z axis.
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    section.rectangle(zx_at(6.0, 0.0), zx_at(10.0, 4.0));
    sketch(b, "section", zx(), section)?;
    revolve(b, "washer", "section", axis(FrameAxis::Z), 360.0, Combine::NewBody)
}
