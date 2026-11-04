//! A closed hollow dome — the disc of radius 20 from z = −2 to z = 0 and on it the hemispherical
//! shell between 18 and 20 from the origin — with a post inside: the disc of radius 3 on the
//! cavity's floor extruded up until it meets the inside of the dome, and joined to it. The dome is
//! turned from its section in the xz plane, the plate's rectangle and the shell's quarter annulus
//! together, about the z axis (geop has no ball of its own); the post is a circle on the xy
//! plane, the cavity's floor, extruded "up to next": until it meets the dome it is joined to.
use crate::shared::{Drawing, axis, made, revolve, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::sketch::whole_sketch;
use geop_ops_model::{Combine, Extent};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let p = [
        zx_at(0.0, -2.0),
        zx_at(20.0, -2.0),
        zx_at(20.0, 0.0),
        zx_at(0.0, 20.0),
        zx_at(0.0, 18.0),
        zx_at(18.0, 0.0),
        zx_at(0.0, 0.0),
    ]
    .map(|at| section.point(at));
    section.line(p[0], p[1]);
    section.line(p[1], p[2]);
    // The shell's outside, a quarter circle up from (20, 0) to (0, 20): clockwise in the sketch,
    // whose second axis is −z.
    section.arc(p[2], p[3], -90.0);
    section.line(p[3], p[4]);
    // Its inside, back down from (0, 18) to (18, 0).
    section.arc(p[4], p[5], 90.0);
    section.line(p[5], p[6]);
    section.line(p[6], p[0]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "dome", "section", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;

    let mut foot = Drawing::new();
    foot.circle([0.0, 0.0], 3.0);
    sketch(b, "foot", xy(), foot)?;
    b.step(
        "post",
        ExtrudeArgs {
            sketch: whole_sketch("foot"),
            extent: Extents { side1: Extent::UpToNext, ..Extents::blind(1.0) },
            output: Output::Solid(Combine::join(made("revolve", "dome"))),
            draft: 0.0,
        },
    )
}
