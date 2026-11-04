//! The box from the origin to (20, 12, 10), its side x = 20 drafted 10° about the plane of its
//! floor, pulled along +z, so it leans in (toward −x) as it rises: the box extruded from a
//! rectangle, and a draft of that one side about the xy plane.
use crate::shared::blends::draft;
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [20.0, 12.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "box", "base", 10.0, Combine::NewBody)?;
    let sides = b.faces(&[[20.0, 6.0, 5.0]])?;
    b.step("taper", draft(sides, xy(), 10.0))
}
