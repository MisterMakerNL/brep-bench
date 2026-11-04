//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x rounded
//! to radius 2, half the top's width, so that the top face vanishes and the block's top is a half
//! cylinder: the block extruded, and one fillet of both edges.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 4.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "slab", "base", 6.0, Combine::NewBody)?;
    let edges = b.edges(&[[5.0, 0.0, 6.0], [5.0, 4.0, 6.0]])?;
    b.step("round", FilletArgs::constant(edges, 2.0))
}
