//! A ball of radius 5 centred on the origin united with a cylinder of the same radius on the z
//! axis from z = −3 to z = 3, the sphere touching the wall from inside along the equator. geop
//! has no ball of its own: a half disc in the xz plane is turned about the z axis. The band is a
//! circle of radius 5 on the xy plane extruded 3 each way; then the boolean union.
use crate::shared::booleans::{ball, boolean};
use crate::shared::{Drawing, extrude_symmetric, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let ball = ball(b, "ball", [0.0, 0.0, 0.0], 5.0)?;
    let mut rim = Drawing::new();
    rim.circle([0.0, 0.0], 5.0);
    sketch(b, "rim", xy(), rim)?;
    extrude_symmetric(b, "band", "rim", 6.0, Combine::NewBody)?;
    boolean(b, "union", &ball, &made("extrude", "band"), BooleanOp::Union)
}
