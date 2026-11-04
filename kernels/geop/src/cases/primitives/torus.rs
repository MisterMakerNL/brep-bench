//! A solid torus centred on the origin, about the z axis, the circle through the middle of its
//! tube of radius 10 and the tube of radius 2: the circle of radius 2 about (10, 0) in the xz
//! plane, revolved a full turn about the z axis. geop has no torus of its own: a torus is turned.
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(10.0, 0.0), 2.0);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)
}
