//! A solid ellipsoid centred on the origin, its semi-axes 6 along x, 4 along y and 3 along z. geop
//! has no ellipsoid (nor ball): a ball of radius 1 is turned from a half disc about the z axis,
//! then scaled by 6, 4 and 3 along x, y and z, which geop does exactly, on the control points.
use crate::shared::{Drawing, made, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;
use geop_ops_pattern::{ScaleBodyArgs, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let (bottom, top) = (half.point(zx_at(0.0, -1.0)), half.point(zx_at(0.0, 1.0)));
    let on_axis = half.line(bottom, top);
    half.arc(top, bottom, 180.0);
    sketch(b, "half", zx(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)?;
    let ball = body(made("revolve", "ball"));
    b.step(
        "stretch",
        ScaleBodyArgs { bodies: vec![ball], about: [0.0; 3], factors: [6.0, 4.0, 3.0] },
    )
}
