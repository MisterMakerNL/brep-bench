//! The block from the origin to (40, 30, 15) with a pocket from (8, 8, 5) to (32, 22, 15), its
//! vertical edges rounded to radius 4, cut into its top; all eight walls drafted 5° about the
//! pocket's floor, z = 5, pulled along +z, so the pocket widens toward its opening, the quarter
//! cylinders becoming quarter cones about their own axes. geop drafts only planar faces, and none
//! that meets a neighbour tangentially, so the pocket is drafted as it is cut: the rounded
//! rectangle sketched on a datum plane at z = 5 and extruded 15 up, past the block's top, with
//! its walls drafted −5° (widening as they rise, a line's wall a plane and an arc's a cone about
//! its axis), and cut from the block.
use crate::shared::blends::{offset_plane, rounded_rectangle};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::Combine;
use geop_ops_model::sketch::whole_sketch;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [40.0, 30.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 15.0, Combine::NewBody)?;
    let floor = offset_plane(b, "floor", FrameAxis::Z, 5.0)?;
    let mut mouth = Drawing::new();
    rounded_rectangle(&mut mouth, [8.0, 8.0], [32.0, 22.0], 4.0);
    sketch(b, "mouth", floor, mouth)?;
    b.step(
        "pocket",
        ExtrudeArgs {
            sketch: whole_sketch("mouth"),
            extent: Extents::blind(15.0),
            output: Output::Solid(Combine::cut(made("extrude", "block"))),
            draft: -5.0,
        },
    )
}
