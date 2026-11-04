//! Two blocks 10 high standing on the xy plane, the first over x and y from 0 to 10, the second
//! from 5 to 15. Make the part common to both: two rectangles sketched on the xy plane, each extruded 10 up as a
//! body of its own, and the boolean intersection of the two.
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
    second.rectangle([5.0, 5.0], [15.0, 15.0]);
    sketch(b, "second", xy(), second)?;
    extrude(b, "b", "second", 10.0, Combine::NewBody)?;
    boolean(b, "common", &made("extrude", "a"), &made("extrude", "b"), BooleanOp::Intersection)
}
