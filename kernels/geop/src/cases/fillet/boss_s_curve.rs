//! A square plate from (−20, −20, 0) to (20, 20, 4) and a round boss of radius 5 on it up to
//! z = 8, joined; both circular edges of the boss rounded to radius 2, the rim at z = 8 cut and
//! the root at z = 4 filled, the two rounds meeting tangentially at z = 6 so the boss's profile is
//! an S. The plate is extruded from a square, the boss from a circle on the xy plane up to z = 8
//! and joined to it, and both edges filleted in one step.
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut square = Drawing::new();
    square.rectangle([-20.0, -20.0], [20.0, 20.0]);
    sketch(b, "square", xy(), square)?;
    extrude(b, "plate", "square", 4.0, Combine::NewBody)?;

    let mut foot = Drawing::new();
    foot.circle([0.0, 0.0], 5.0);
    sketch(b, "foot", xy(), foot)?;
    extrude(b, "boss", "foot", 8.0, Combine::join(made("extrude", "plate")))?;

    let edges = b.edges(&[[3.0, 4.0, 8.0], [3.0, 4.0, 4.0]])?;
    b.step("round", FilletArgs::constant(edges, 2.0))
}
