//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 with everything below z = 0 cut away: a dome on the
//! flat ellipse of semi-axes 6 and 4. geop has no ellipsoid (nor ball): a ball of radius 1 is
//! turned from a half disc about the z axis and scaled by 6, 4 and 3 along x, y and z; then a
//! block from (−10, −10, −10) to (10, 10, 0), reaching past it everywhere but at z = 0, is cut
//! from it.
use crate::shared::{Drawing, extrude, made, revolve, sketch, sketch_axis, xy, zx, zx_at};
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
    let ball = made("revolve", "ball");
    b.step(
        "stretch",
        ScaleBodyArgs { bodies: vec![body(&ball)], about: [0.0; 3], factors: [6.0, 4.0, 3.0] },
    )?;
    let mut below = Drawing::new();
    below.rectangle([-10.0, -10.0], [10.0, 10.0]);
    sketch(b, "below", xy(), below)?;
    extrude(b, "cut", "below", -10.0, Combine::cut(ball))
}
