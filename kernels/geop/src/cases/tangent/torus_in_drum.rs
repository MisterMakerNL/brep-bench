//! A solid torus about the z axis, its tube's middle circle of radius 10 in the xy plane and the
//! tube of radius 3, united with a drum of radius 13 on the z axis from z = −1 to z = 1, whose
//! wall the torus touches from inside along its outer equator. geop has no torus of its own: the
//! circle of radius 3 about (10, 0) in the xz plane is turned about the z axis. The drum is a
//! circle of radius 13 on the xy plane extruded 1 each way; then the boolean union.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, axis, extrude_symmetric, made, revolve, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(10.0, 0.0), 3.0);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;
    let mut rim = Drawing::new();
    rim.circle([0.0, 0.0], 13.0);
    sketch(b, "rim", xy(), rim)?;
    extrude_symmetric(b, "drum", "rim", 2.0, Combine::NewBody)?;
    boolean(b, "union", &made("revolve", "ring"), &made("extrude", "drum"), BooleanOp::Union)
}
