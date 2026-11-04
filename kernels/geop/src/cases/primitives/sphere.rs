//! A solid ball of radius 5 centred on the origin: a half disc in the xz plane, bounded by the
//! z axis from z = −5 to z = 5 and the half circle through x = 5, revolved a full turn about its
//! straight side. geop has no ball of its own: a ball is turned, as a designer turns one.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let bottom = half.point(zx_at(0.0, -5.0));
    let top = half.point(zx_at(0.0, 5.0));
    let on_axis = half.line(bottom, top);
    // From the top, counter-clockwise in the sketch (whose second axis is −z) half a turn,
    // through x = 5, to the bottom.
    half.arc(top, bottom, 180.0);
    sketch(b, "half", zx(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)
}
