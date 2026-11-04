//! A prism from z = 0 to z = 10 whose section is the periodic uniform quadratic B-spline on the
//! control square of corners (±10, ±10): four parabolic arcs, each from the middle of one side to
//! the middle of the next with the corner between as its control point. geop's sketch has no
//! periodic spline: the curve is drawn as those four arcs, each the quadratic spline through its
//! three control points (a Bézier), meeting tangentially; extruded 10.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let corners = [[10.0, 10.0], [-10.0, 10.0], [-10.0, -10.0], [10.0, -10.0]];
    let middles = [[10.0, 0.0], [0.0, 10.0], [-10.0, 0.0], [0.0, -10.0]].map(|m| section.point(m));
    for k in 0..4 {
        let corner = section.point(corners[k]);
        section.sketch.add_spline(vec![middles[k], corner, middles[(k + 1) % 4]]);
    }
    sketch(b, "section", xy(), section)?;
    extrude(b, "prism", "section", 10.0, Combine::NewBody)
}
