//! A solid cone on the disc of radius 5 about the z axis in the plane z = 0, its apex at
//! (0, 0, 10), its side one NURBS face: rational quadratic round the axis — the base circle's
//! nine control points — and linear up it to the apex, where its whole top edge collapses. geop
//! takes no surface by its control net. Its loft from a sketched circle to a sketched point rules
//! the circle — a rational quadratic, in quarter turns — into the apex, which is that surface,
//! and caps the base with the disc; geop makes the side four faces, a quarter turn each.
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{LoftArgs, Output};
use geop_ops_model::Combine;
use geop_ops_model::datums::{datum, origin_plane};
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::{SketchPlane, whole_sketch};
use geop_ops_reference_geometry::{AddDatumArgs, Construction};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 5.0);
    sketch(b, "base", xy(), base)?;
    let xy_plane = Selected::Datum(datum(origin_plane(FrameAxis::Z).name()));
    let construction = Construction::Offset { distance: 10.0.into() };
    b.step("apex_plane", AddDatumArgs { selection: vec![xy_plane], construction })?;
    let mut apex = Drawing::new();
    apex.point([0.0, 0.0]);
    sketch(b, "apex", SketchPlane::Datum(datum("apex_plane")), apex)?;
    b.step(
        "cone",
        LoftArgs {
            profiles: vec![whole_sketch("base"), whole_sketch("apex")],
            matches: Vec::new(),
            guides: Vec::new(),
            output: Output::Solid(Combine::NewBody),
        },
    )
}
