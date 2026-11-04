//! The block from the origin to (40, 30, 15) with a rectangular pocket from (8, 8, 5) to
//! (32, 22, 15) cut into its top; the pocket's four walls drafted 5° about the plane of its
//! floor, z = 5, pulled along +z, so that the pocket widens toward its opening. The block is
//! extruded, the pocket cut by a rectangle on a datum plane at z = 5 extruded up past the top,
//! and the four walls drafted about that plane.
use crate::shared::blends::{draft, offset_plane};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [40.0, 30.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 15.0, Combine::NewBody)?;
    let floor = offset_plane(b, "floor", FrameAxis::Z, 5.0)?;
    let mut mouth = Drawing::new();
    mouth.rectangle([8.0, 8.0], [32.0, 22.0]);
    sketch(b, "mouth", floor.clone(), mouth)?;
    extrude(b, "pocket", "mouth", 15.0, Combine::cut(made("extrude", "block")))?;
    let walls = b.faces(&[[20.0, 8.0, 10.0], [32.0, 15.0, 10.0], [20.0, 22.0, 10.0], [8.0, 15.0, 10.0]])?;
    b.step("taper", draft(walls, floor, 5.0))
}
