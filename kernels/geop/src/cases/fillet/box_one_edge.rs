//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded to radius 2. The edge is picked by a point on it, as a designer clicks it.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let edge = b.edge([10.0, 10.0, 5.0])?;
    b.step("round", FilletArgs::constant(vec![edge], 2.0))
}
