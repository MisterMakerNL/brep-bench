//! A ball of radius 15 centred on the origin united with a solid cone on the z axis, its apex at
//! (0, 0, −25) and its base the disc of radius 12 at z = −9, whose side is tangent to the sphere
//! along the base's rim. geop has no ball or cone of its own: the ball is a half disc in the xz
//! plane turned about the z axis, and the cone the triangle (0, −25), (12, −9), (0, −9) in the xz
//! plane turned about its side on the z axis. Then the boolean union.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, made, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 15.0)?;
    let mut section = Drawing::new();
    let (_, sides) = section.polygon(&[zx_at(0.0, -9.0), zx_at(0.0, -25.0), zx_at(12.0, -9.0)]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "cone", "section", sketch_axis("section", sides[0]), 360.0, Combine::NewBody)?;
    boolean(b, "union", &ball, &made("revolve", "cone"), BooleanOp::Union)
}
