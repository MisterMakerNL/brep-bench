//! A cube of side 10 from (−5, −5, 0) to (5, 5, 10), united with a copy of itself turned 0.001°
//! about the z axis: a square on the xy plane extruded 10 up, and a move of a copy of it, turned
//! about the origin's z axis, joined to it.
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;
use geop_ops_model::datums::origin_axis;
use geop_ops_pattern::{MoveBodyArgs, TurnAxis, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-5.0, -5.0], [5.0, 5.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let cube = made("extrude", "cube");
    b.step(
        "turned",
        MoveBodyArgs {
            bodies: vec![body(&cube)],
            translation: [0.0; 3],
            axis: Some(TurnAxis::Datum(origin_axis(FrameAxis::Z))),
            angle: 0.001,
            copy: true,
            combine: Combine::join(cube),
        },
    )
}
