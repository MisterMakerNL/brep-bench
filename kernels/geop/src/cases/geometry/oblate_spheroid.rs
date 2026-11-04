//! A solid oblate spheroid about the z axis: radius 6 at its equator, reaching to z = ±3 at its
//! poles — the ellipse x²/36 + z²/9 ≤ 1 of the xz plane turned about the z axis. geop's sketch has
//! no ellipse (nor geop a ball): a ball of radius 6 is turned from a half disc about the z axis,
//! then scaled by a half along z, which geop does exactly, on the control points.
use crate::shared::{Drawing, made, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;
use geop_ops_pattern::{ScaleBodyArgs, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let (bottom, top) = (half.point(zx_at(0.0, -6.0)), half.point(zx_at(0.0, 6.0)));
    let on_axis = half.line(bottom, top);
    half.arc(top, bottom, 180.0);
    sketch(b, "half", zx(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)?;
    let ball = body(made("revolve", "ball"));
    b.step(
        "flatten",
        ScaleBodyArgs { bodies: vec![ball], about: [0.0; 3], factors: [1.0, 1.0, 0.5] },
    )
}
