//! A ball of radius 5 centred on the origin, with everything on the side x < 0 of the yz plane
//! cut away, through its poles. geop has no ball of its own: a half disc in the xz plane is
//! turned about the z axis, so its poles are at (0, 0, ±5). The half where x < 0 is cut away by
//! a block over x from −10 to 0 and y from −10 to 10, extruded 10 each way along z.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude_symmetric, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 5.0)?;
    let mut half = Drawing::new();
    half.rectangle([-10.0, -10.0], [0.0, 10.0]);
    sketch(b, "half", xy(), half)?;
    extrude_symmetric(b, "block", "half", 20.0, Combine::NewBody)?;
    boolean(b, "cut", &ball, &made("extrude", "block"), BooleanOp::Difference)
}
