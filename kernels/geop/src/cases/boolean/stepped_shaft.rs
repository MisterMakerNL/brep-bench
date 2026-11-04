//! A stepped shaft on the z axis, one solid: a cylinder of radius 5 from z = 0 to z = 10, and on
//! it one of radius 3 from z = 10 to z = 20. A circle of radius 5 on the xy plane extruded 10
//! up; a circle of radius 3 on the datum plane 10 above it extruded 10 up as a body of its own;
//! and the boolean union of the two.
use crate::shared::booleans::{boolean, offset_plane};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut wide = Drawing::new();
    wide.circle([0.0, 0.0], 5.0);
    sketch(b, "wide", xy(), wide)?;
    extrude(b, "first", "wide", 10.0, Combine::NewBody)?;
    let step = offset_plane(b, "step", FrameAxis::Z, 10.0)?;
    let mut narrow = Drawing::new();
    narrow.circle([0.0, 0.0], 3.0);
    sketch(b, "narrow", step, narrow)?;
    extrude(b, "second", "narrow", 10.0, Combine::NewBody)?;
    boolean(b, "shaft", &made("extrude", "first"), &made("extrude", "second"), BooleanOp::Union)
}
