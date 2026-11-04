//! A solid cylinder of radius 5 and height 10 whose axis is the z axis, standing on the xy plane:
//! a circle of radius 5 about the origin, sketched on the xy plane and extruded 10 up.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 5.0);
    sketch(b, "base", xy(), base)?;
    extrude(b, "drum", "base", 10.0, Combine::NewBody)
}
