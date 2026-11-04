//! A cube of side 10 between the origin and (10, 10, 10), with a hole of radius 3 drilled through
//! it along z at x = y = 5: a square on the xy plane extruded 10 up, and a circle of radius 3 on
//! the same plane extruded from z = −1 to z = 11, past both faces, cut from it.
use crate::shared::booleans::extrude_between;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let mut bore = Drawing::new();
    bore.circle([5.0, 5.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_between(b, "hole", "bore", -1.0, 11.0, Combine::cut(made("extrude", "cube")))
}
