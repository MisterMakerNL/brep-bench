//! A block from (−10, −10, −10) to (10, 10, 0), with a ball of radius 5 centred on the origin cut
//! from it, leaving a hemispherical pocket in its top: a square on the xy plane extruded 10
//! down, and the ball — geop has none of its own: a half disc in the xz plane turned about the
//! z axis — cut from it by a boolean difference.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-10.0, -10.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", -10.0, Combine::NewBody)?;
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 5.0)?;
    boolean(b, "pocket", &made("extrude", "block"), &ball, BooleanOp::Difference)
}
