//! A ball of radius 5 centred on the origin, less the cap beyond the plane x + z = 5, which
//! leans at 45° and passes through the pole (0, 0, 5). geop has no ball of its own: a half disc
//! in the xz plane is turned about the z axis, so its poles are on the z axis. The cap is cut
//! away by a block drawn on the xz plane, the triangle on the far side of the line x + z = 5
//! from (7, −2) to (−2, 7), extruded 6 each way along y, past the ball.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude_symmetric, made, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 5.0)?;
    let mut beyond = Drawing::new();
    beyond.polygon(&[zx_at(7.0, -2.0), zx_at(12.0, 12.0), zx_at(-2.0, 7.0)]);
    sketch(b, "beyond", zx(), beyond)?;
    extrude_symmetric(b, "block", "beyond", 12.0, Combine::NewBody)?;
    boolean(b, "cut", &ball, &made("extrude", "block"), BooleanOp::Difference)
}
