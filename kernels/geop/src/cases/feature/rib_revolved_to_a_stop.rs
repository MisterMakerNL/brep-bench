//! A hub, the cylinder of radius 10 on the z axis from z = 0 to z = 10; a stop joined to it, the
//! wedge within 25 of the axis between the half-planes at 120° and 135°, 10 tall; and a rib, the
//! rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 in the plane y = 0, turned about the z axis from 0° towards +y
//! until it meets the stop's face at 120°, and joined to them. The hub and the stop are extruded
//! from the xy plane, the rib's rectangle sketched on the xz plane and revolved about the z axis
//! "up to next": until it meets the solid it is joined to.
use crate::shared::{Drawing, axis, extrude, made, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{Extents, Output, RevolveArgs};
use geop_ops_model::sketch::whole_sketch;
use geop_ops_model::{Combine, Extent};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 10.0);
    sketch(b, "base", xy(), base)?;
    extrude(b, "hub", "base", 10.0, Combine::NewBody)?;

    let mut wedge = Drawing::new();
    let at = |deg: f64| [25.0 * deg.to_radians().cos(), 25.0 * deg.to_radians().sin()];
    let centre = wedge.point([0.0, 0.0]);
    let from = wedge.point(at(120.0));
    let to = wedge.point(at(135.0));
    wedge.line(centre, from);
    wedge.arc(from, to, 15.0);
    wedge.line(to, centre);
    sketch(b, "wedge", xy(), wedge)?;
    extrude(b, "stop", "wedge", 10.0, Combine::join(made("extrude", "hub")))?;

    let mut section = Drawing::new();
    section.rectangle(zx_at(5.0, 0.0), zx_at(20.0, 4.0));
    sketch(b, "section", zx(), section)?;
    b.step(
        "rib",
        RevolveArgs {
            sketch: whole_sketch("section"),
            axis: Some(axis(FrameAxis::Z)),
            extent: Extents { side1: Extent::UpToNext, ..Extents::blind(1.0) },
            output: Output::Solid(Combine::join(made("extrude", "stop"))),
        },
    )
}
