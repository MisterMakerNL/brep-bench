//! A square plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 through it on the z
//! axis; both rims of the hole rounded to radius 1, half the plate's thickness, so that the two
//! rounds meet at the waist and the hole's wall vanishes. The plate is extruded from a square, the
//! hole cut by a circle extruded past both faces, and both rims filleted in one step.
use crate::shared::{Drawing, extrude, extrude_symmetric, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut square = Drawing::new();
    square.rectangle([-10.0, -10.0], [10.0, 10.0]);
    sketch(b, "square", xy(), square)?;
    extrude(b, "plate", "square", 2.0, Combine::NewBody)?;

    let mut bore = Drawing::new();
    bore.circle([0.0, 0.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 10.0, Combine::cut(made("extrude", "plate")))?;

    let rims = b.edges(&[[1.8, 2.4, 2.0], [1.8, 2.4, 0.0]])?;
    b.step("round", FilletArgs::constant(rims, 1.0))
}
