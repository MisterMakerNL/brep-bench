//! A ring about the z axis whose section, in the xz plane, is the periodic uniform quadratic
//! B-spline on the control square of corners (13, 3), (7, 3), (7, −3) and (13, −3): four
//! parabolic arcs, each from the middle of one side to the middle of the next with the corner
//! between as its control point, turned a whole turn about the axis. geop's sketch has no periodic
//! spline: the curve is drawn as those four arcs, each the quadratic spline through its three
//! control points (a Bézier), meeting tangentially; revolved about the z axis.
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let corners = [(13.0, 3.0), (7.0, 3.0), (7.0, -3.0), (13.0, -3.0)];
    let middles = [(13.0, 0.0), (10.0, 3.0), (7.0, 0.0), (10.0, -3.0)]
        .map(|(x, z)| section.point(zx_at(x, z)));
    for k in 0..4 {
        let corner = section.point(zx_at(corners[k].0, corners[k].1));
        section.sketch.add_spline(vec![middles[k], corner, middles[(k + 1) % 4]]);
    }
    sketch(b, "section", zx(), section)?;
    revolve(b, "ring", "section", axis(FrameAxis::Z), 360.0, Combine::NewBody)
}
