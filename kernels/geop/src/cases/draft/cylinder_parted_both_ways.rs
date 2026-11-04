//! A cylinder of radius 10 about the z axis from z = 0 to z = 20, its side split at the parting
//! line z = 10 and each half drafted 3° away from it, pulled +z above and −z below: two frustums
//! of cones joined at their wide ends, radius 10 at z = 10. geop drafts only planar faces, so the
//! cylinder is drafted as it is made: the circle of radius 10 sketched on a datum plane at the
//! parting line and extruded 10 each way with its wall drafted 3°, which leans the wall in away
//! from the sketch's plane on both sides, a cone about its axis, with a ridge at the plane.
use crate::shared::blends::offset_plane;
use crate::shared::{Drawing, sketch};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::Combine;
use geop_ops_model::sketch::whole_sketch;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let parting = offset_plane(b, "parting", FrameAxis::Z, 10.0)?;
    let mut rim = Drawing::new();
    rim.circle([0.0, 0.0], 10.0);
    sketch(b, "rim", parting, rim)?;
    b.step(
        "body",
        ExtrudeArgs {
            sketch: whole_sketch("rim"),
            extent: Extents { symmetric: true, ..Extents::blind(20.0) },
            output: Output::Solid(Combine::NewBody),
            draft: 3.0,
        },
    )
}
