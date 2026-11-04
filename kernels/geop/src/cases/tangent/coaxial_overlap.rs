//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from z = 6 to 16, united: their
//! walls coincide from z = 6 to 10. Each is a circle of radius 5 extruded 10 up, the first from
//! the xy plane, the second from the datum plane 6 above it. A geop cylinder has four seams,
//! where its circle's quarters meet, so a quarter turn would bring the second's onto the
//! first's: it is turned an eighth of a turn about the z axis instead, so that the coincident
//! walls are not split in the same place. Then the boolean union.
use crate::shared::booleans::{boolean, offset_plane};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::datums::origin_axis;
use geop_ops_model::{BooleanOp, Combine};
use geop_ops_pattern::{MoveBodyArgs, TurnAxis, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut lower = Drawing::new();
    lower.circle([0.0, 0.0], 5.0);
    sketch(b, "lower", xy(), lower)?;
    extrude(b, "first", "lower", 10.0, Combine::NewBody)?;
    let six = offset_plane(b, "six", FrameAxis::Z, 6.0)?;
    let mut upper = Drawing::new();
    upper.circle([0.0, 0.0], 5.0);
    sketch(b, "upper", six, upper)?;
    extrude(b, "second", "upper", 10.0, Combine::NewBody)?;
    b.step(
        "turn",
        MoveBodyArgs {
            bodies: vec![body(made("extrude", "second"))],
            translation: [0.0; 3],
            axis: Some(TurnAxis::Datum(origin_axis(FrameAxis::Z))),
            angle: 45.0,
            copy: false,
            combine: Combine::NewBody,
        },
    )?;
    boolean(b, "union", &made("extrude", "first"), &made("extrude", "second"), BooleanOp::Union)
}
