//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x each
//! chamfered with distance 2 on both faces, half the top's width, so that the top face vanishes
//! and the chamfers meet in a ridge along y = 2, z = 6: the block extruded, and one chamfer of
//! both edges.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::ChamferArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 4.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 6.0, Combine::NewBody)?;
    let edges = b.edges(&[[5.0, 0.0, 6.0], [5.0, 4.0, 6.0]])?;
    b.step("bevel", ChamferArgs::even(edges, 2.0))
}
