//! A cube of side 10 between the origin and (10, 10, 10) and the ball of radius 5 inscribed in
//! it, centred on (5, 5, 5), touching each face at its middle. Their union, the cube itself: the cube a square on
//! the xy plane extruded 10 up; the ball — geop has none of its own — a half disc on the datum
//! plane y = 5 turned about the vertical through the centre; then the boolean union.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let ball = ball(b, "ball", [5.0, 5.0, 5.0], 5.0)?;
    boolean(b, "union", &made("extrude", "cube"), &ball, BooleanOp::Union)
}
