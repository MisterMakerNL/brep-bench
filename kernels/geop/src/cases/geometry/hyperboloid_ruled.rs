//! The solid bounded by two discs of radius 5 about the z axis, at z = −5 and z = 5, and the
//! surface ruled between their rims, the point at angle θ below joined to the point at θ + 90°
//! above. Built as the case gives it: the two rims are circles sketched on the xy plane moved 5
//! down and 5 up, and lofted into each other — two profiles, so by ruled walls, and capped by
//! the discs — with the point at angle 0 on the lower matched to the point at 90° on the upper.
use crate::shared::{Drawing, sketch};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{LoftArgs, Output};
use geop_ops_model::datums::{datum, origin_plane};
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::{SketchPlane, sketch_element, whole_sketch};
use geop_ops_model::Combine;
use geop_ops_reference_geometry::{AddDatumArgs, Construction};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    for (id, distance) in [("below", -5.0), ("above", 5.0)] {
        let xy = Selected::Datum(datum(origin_plane(FrameAxis::Z).name()));
        let construction = Construction::Offset { distance: distance.into() };
        b.step(id, AddDatumArgs { selection: vec![xy], construction })?;
    }
    let mut lower = Drawing::new();
    lower.circle([0.0, 0.0], 5.0);
    let lower_start = lower.point([5.0, 0.0]);
    sketch(b, "lower", SketchPlane::Datum(datum("below")), lower)?;
    let mut upper = Drawing::new();
    upper.circle([0.0, 0.0], 5.0);
    let upper_start = upper.point([0.0, 5.0]);
    sketch(b, "upper", SketchPlane::Datum(datum("above")), upper)?;
    b.step(
        "solid",
        LoftArgs {
            profiles: vec![whole_sketch("lower"), whole_sketch("upper")],
            matches: vec![
                sketch_element("lower", lower_start),
                sketch_element("upper", upper_start),
            ],
            guides: Vec::new(),
            output: Output::Solid(Combine::NewBody),
        },
    )
}
