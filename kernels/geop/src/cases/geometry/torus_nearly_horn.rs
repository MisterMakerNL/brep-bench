//! A solid torus about the z axis whose tube, of radius 4.99, runs about a middle circle of radius
//! 5: its hole is a circle of radius 0.01. geop has no torus: the disc of radius 4.99 about
//! (5, 0) in the xz plane is revolved a whole turn about the z axis.
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(5.0, 0.0), 4.99);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)
}
