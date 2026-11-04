//! A cube of side 10 between the origin and (10, 10, 10), united with a cylinder of radius 5 and
//! height 10 standing on the xy plane with its axis at x = 10, y = 5, half of it inside the cube
//! and its wall touching the faces y = 0 and y = 10 along the cube's edges at x = 10: a square
//! and a circle sketched on the xy plane, each extruded 10 up as a body of its own, and the
//! boolean union of the two.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let mut round = Drawing::new();
    round.circle([10.0, 5.0], 5.0);
    sketch(b, "round", xy(), round)?;
    extrude(b, "boss", "round", 10.0, Combine::NewBody)?;
    boolean(b, "union", &made("extrude", "cube"), &made("extrude", "boss"), BooleanOp::Union)
}
