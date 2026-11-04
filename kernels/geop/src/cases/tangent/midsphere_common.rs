//! A cube of side 10 from (−5, −5, −5) to (5, 5, 5) and the ball centred on the origin of radius
//! 5√2, whose sphere touches each of the cube's edges at its midpoint; their common part. The
//! cube is a square on the xy plane extruded 5 each way; the ball — geop has none of its own —
//! a half disc in the xz plane turned about the z axis; then the boolean intersection.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude_symmetric, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-5.0, -5.0], [5.0, 5.0]);
    sketch(b, "base", xy(), base)?;
    extrude_symmetric(b, "cube", "base", 10.0, Combine::NewBody)?;
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 5.0 * 2f64.sqrt())?;
    boolean(b, "common", &made("extrude", "cube"), &ball, BooleanOp::Intersection)
}
