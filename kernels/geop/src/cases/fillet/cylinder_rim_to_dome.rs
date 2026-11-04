//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circular edge around its top
//! face rounded to radius 5, the cylinder's own: the top face shrinks to a point and the round is
//! a hemisphere. A circle extruded, and the rim filleted to the full radius.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 5.0);
    sketch(b, "base", xy(), base)?;
    extrude(b, "drum", "base", 10.0, Combine::NewBody)?;
    let rim = b.edge([3.0, 4.0, 10.0])?;
    b.step("dome", FilletArgs::constant(vec![rim], 5.0))
}
