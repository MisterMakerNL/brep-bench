//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, bored along its axis by a hole of
//! radius 3: a circle on the xy plane extruded 10 up, and a circle of radius 3 on the same plane
//! extruded from z = −1 to z = 11, past both ends, cut from it.
use crate::shared::booleans::extrude_between;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outer = Drawing::new();
    outer.circle([0.0, 0.0], 5.0);
    sketch(b, "outer", xy(), outer)?;
    extrude(b, "drum", "outer", 10.0, Combine::NewBody)?;
    let mut bore = Drawing::new();
    bore.circle([0.0, 0.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_between(b, "hole", "bore", -1.0, 11.0, Combine::cut(made("extrude", "drum")))
}
