//! The box from the origin to (20, 12, 10), its four sides drafted 5° about the plane of its
//! floor, pulled along +z, so each leans in as it rises: the box extruded from a rectangle, and
//! one draft of its four sides, each picked by a point on it, about the xy plane.
use crate::shared::blends::draft;
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [20.0, 12.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "box", "base", 10.0, Combine::NewBody)?;
    let sides = b.faces(&[[0.0, 6.0, 5.0], [20.0, 6.0, 5.0], [10.0, 0.0, 5.0], [10.0, 12.0, 5.0]])?;
    b.step("taper", draft(sides, xy(), 5.0))
}
