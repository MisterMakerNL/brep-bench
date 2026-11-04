//! A ball of radius 20 centred on the origin, bored right through parallel to the y axis by a
//! hole of radius 6 whose axis is the line x = 5, z = 0. geop has no ball of its own: a half
//! disc in the xz plane is turned about the z axis. The hole is a circle of radius 6 about
//! (5, 0) in the xz plane, extruded 25 each way along y, past the ball at both ends, and cut.
use crate::shared::booleans::ball;
use crate::shared::{Drawing, extrude_symmetric, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 20.0)?;
    let mut bore = Drawing::new();
    bore.circle(zx_at(5.0, 0.0), 6.0);
    sketch(b, "bore", zx(), bore)?;
    extrude_symmetric(b, "hole", "bore", 50.0, Combine::cut(ball))
}
