//! A rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin and the
//! opposite corner at (10, 20, 30): a rectangle sketched on the xy plane and extruded 30 up.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 20.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 30.0, Combine::NewBody)
}
