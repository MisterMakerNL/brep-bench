//! A block from (−10, −6, 0) to (10, 6, 5) with its four vertical edges rounded to radius 3, the
//! rim of its top face rounded to radius 1. The rounded rectangle is sketched and extruded 5 up;
//! the rim's eight edges run tangentially into one another, so one of them picked takes the whole
//! chain along, and one fillet rounds it all unbroken.
use crate::shared::blends::rounded_rectangle;
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    rounded_rectangle(&mut outline, [-10.0, -6.0], [10.0, 6.0], 3.0);
    sketch(b, "outline", xy(), outline)?;
    extrude(b, "block", "outline", 5.0, Combine::NewBody)?;
    let rim = b.edge([0.0, -6.0, 5.0])?;
    b.step("round", FilletArgs::constant(vec![rim], 1.0))
}
