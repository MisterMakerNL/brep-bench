//! A solid horn torus about the z axis: the circle through the middle of its tube has radius 5,
//! and the tube radius 5 too, so it closes its hole to the origin. geop has no torus: the disc of
//! radius 5 about (5, 0) in the xz plane, which touches the z axis at the origin, is revolved a
//! whole turn about the z axis. geop revolves a curve that touches the axis only where it ends
//! there, so the circle is drawn as two half circles meeting at the origin.
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    let (pinch, outer) = (tube.point(zx_at(0.0, 0.0)), tube.point(zx_at(10.0, 0.0)));
    tube.arc(pinch, outer, 180.0);
    tube.arc(outer, pinch, 180.0);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)
}
