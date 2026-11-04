//! A ball of radius 5 centred on the origin, with everything below the xy plane cut away. geop
//! has no ball of its own: a half disc in the xz plane is turned about the z axis. Below the xy
//! plane, a block over x and y from −10 to 10, extruded 10 down from it, is cut from the ball.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 5.0)?;
    let mut below = Drawing::new();
    below.rectangle([-10.0, -10.0], [10.0, 10.0]);
    sketch(b, "below", xy(), below)?;
    extrude(b, "block", "below", -10.0, Combine::NewBody)?;
    boolean(b, "cut", &ball, &made("extrude", "block"), BooleanOp::Difference)
}
