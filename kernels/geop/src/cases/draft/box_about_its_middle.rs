//! The box from the origin to (20, 12, 10), its four sides drafted 5° about the plane z = 5,
//! halfway up, pulled along +z: each turns about its line at z = 5, out below it and in above.
//! The box is extruded, a datum plane offset 5 from the xy plane, and one draft of the four sides
//! about it.
use crate::shared::blends::{draft, offset_plane};
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [20.0, 12.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "box", "base", 10.0, Combine::NewBody)?;
    let middle = offset_plane(b, "middle", FrameAxis::Z, 5.0)?;
    let sides = b.faces(&[[0.0, 6.0, 5.0], [20.0, 6.0, 5.0], [10.0, 0.0, 5.0], [10.0, 12.0, 5.0]])?;
    b.step("taper", draft(sides, middle, 5.0))
}
