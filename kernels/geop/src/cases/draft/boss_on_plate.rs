//! A square plate from (−20, −20, 0) to (20, 20, 5) and on it a boss of radius 8 about the z axis
//! up to z = 17, its side drafted 4° about the plate's top, pulled along +z: a frustum of a cone,
//! radius 8 at z = 5. geop drafts only planar faces, so the boss is drafted as it is made: the
//! circle sketched on a datum plane at z = 5 and extruded 12 up with its wall drafted 4°, which
//! leans an arc's wall in as a cone about its axis, and joined to the plate.
use crate::shared::blends::offset_plane;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::Combine;
use geop_ops_model::sketch::whole_sketch;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut square = Drawing::new();
    square.rectangle([-20.0, -20.0], [20.0, 20.0]);
    sketch(b, "square", xy(), square)?;
    extrude(b, "plate", "square", 5.0, Combine::NewBody)?;
    let top = offset_plane(b, "top", FrameAxis::Z, 5.0)?;
    let mut foot = Drawing::new();
    foot.circle([0.0, 0.0], 8.0);
    sketch(b, "foot", top, foot)?;
    b.step(
        "boss",
        ExtrudeArgs {
            sketch: whole_sketch("foot"),
            extent: Extents::blind(12.0),
            output: Output::Solid(Combine::join(made("extrude", "plate"))),
            draft: 4.0,
        },
    )
}
