//! The box from the origin to (30, 10, 10), its four sides drafted 30° about the plane of its
//! floor, pulled along +z, so steeply that the long sides meet below the top in a ridge and the
//! draft consumes the top face, leaving a hip roof: the box extruded, and one draft of its four
//! sides about the xy plane.
use crate::shared::blends::draft;
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [30.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "box", "base", 10.0, Combine::NewBody)?;
    let sides = b.faces(&[[0.0, 5.0, 5.0], [30.0, 5.0, 5.0], [15.0, 0.0, 5.0], [15.0, 10.0, 5.0]])?;
    b.step("taper", draft(sides, xy(), 30.0))
}
