//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 that run along x
//! rounded to radius 0.9999, leaving of that face a strip only 0.0002 tall between the rounds:
//! the plate extruded, and one fillet of both edges.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [20.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "plate", "base", 2.0, Combine::NewBody)?;
    let edges = b.edges(&[[10.0, 10.0, 0.0], [10.0, 10.0, 2.0]])?;
    b.step("round", FilletArgs::constant(edges, 0.9999))
}
