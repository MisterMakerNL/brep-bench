//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, less a ball of radius 5 centred on
//! the middle of its top face, (0, 0, 10), whose equator is the top's rim: a circle on the xy
//! plane extruded 10 up, and the ball — geop has none of its own: a half disc in the xz plane
//! turned about the z axis — cut from it by a boolean difference.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 5.0);
    sketch(b, "base", xy(), base)?;
    extrude(b, "drum", "base", 10.0, Combine::NewBody)?;
    let ball = ball(b, "ball", [0.0, 0.0, 10.0], 5.0)?;
    boolean(b, "dish", &made("extrude", "drum"), &ball, BooleanOp::Difference)
}
