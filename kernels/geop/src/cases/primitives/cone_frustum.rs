//! A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base and 2 at its
//! top: its half section in the xz plane, the trapezoid (0, 0), (5, 0), (2, 6), (0, 6), revolved
//! a full turn about the z axis. geop has no cone of its own: a cone is turned.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let (_, sides) =
        section.polygon(&[zx_at(0.0, 6.0), zx_at(0.0, 0.0), zx_at(5.0, 0.0), zx_at(2.0, 6.0)]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "frustum", "section", sketch_axis("section", sides[0]), 360.0, Combine::NewBody)
}
