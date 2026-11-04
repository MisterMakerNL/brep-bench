//! Two cubes of side 10 joined into one solid: the first between the origin and (10, 10, 10), the
//! second between (5, 0, 10) and (15, 10, 20), standing on half the first's top. The first is a
//! square on the xy plane extruded 10 up; the second a square on the datum plane 10 above it,
//! extruded 10 up as a body of its own; and the boolean union of the two.
use crate::shared::booleans::{boolean, offset_plane};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut first = Drawing::new();
    first.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "first", xy(), first)?;
    extrude(b, "a", "first", 10.0, Combine::NewBody)?;
    let level = offset_plane(b, "level", FrameAxis::Z, 10.0)?;
    let mut second = Drawing::new();
    second.rectangle([5.0, 0.0], [15.0, 10.0]);
    sketch(b, "second", level, second)?;
    extrude(b, "b", "second", 10.0, Combine::NewBody)?;
    boolean(b, "union", &made("extrude", "a"), &made("extrude", "b"), BooleanOp::Union)
}
