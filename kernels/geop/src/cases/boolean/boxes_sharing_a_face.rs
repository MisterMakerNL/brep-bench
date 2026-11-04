//! Two cubes of side 10 side by side, the first between the origin and (10, 10, 10), the second
//! between (10, 0, 0) and (20, 10, 10), joined into one solid: two squares on the xy plane, each
//! extruded 10 up as a body of its own, and their boolean union, where the face they share goes.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut first = Drawing::new();
    first.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "first", xy(), first)?;
    extrude(b, "a", "first", 10.0, Combine::NewBody)?;
    let mut second = Drawing::new();
    second.rectangle([10.0, 0.0], [20.0, 10.0]);
    sketch(b, "second", xy(), second)?;
    extrude(b, "b", "second", 10.0, Combine::NewBody)?;
    boolean(b, "union", &made("extrude", "a"), &made("extrude", "b"), BooleanOp::Union)
}
