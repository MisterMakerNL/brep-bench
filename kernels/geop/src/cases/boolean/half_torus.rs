//! A torus about the z axis, centred on the origin, its tube's middle circle of radius 10 and its
//! tube of radius 2, with everything where x < 0 cut away. geop has no torus of its own: the
//! circle of radius 2 about (10, 0) in the xz plane is turned a full turn about the z axis. The
//! half where x < 0 is cut away by a block over x from −15 to 0 and y from −15 to 15, extruded
//! from z = −5 to z = 5, past the torus all round.
use crate::shared::booleans::{boolean, extrude_between};
use crate::shared::{Drawing, axis, made, revolve, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(10.0, 0.0), 2.0);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;
    let mut half = Drawing::new();
    half.rectangle([-15.0, -15.0], [0.0, 15.0]);
    sketch(b, "half", xy(), half)?;
    extrude_between(b, "block", "half", -5.0, 5.0, Combine::NewBody)?;
    boolean(b, "cut", &made("revolve", "ring"), &made("extrude", "block"), BooleanOp::Difference)
}
