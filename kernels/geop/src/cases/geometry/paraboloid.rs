//! The solid paraboloid of revolution x² + y² ≤ 8z, z ≤ 8: the parabola x² = 8z of the xz plane
//! from its vertex at the origin to (8, 8), the line across at z = 8 and the z axis, turned a
//! whole turn about the axis. geop's sketch has no parabola: the arc is the quadratic B-spline
//! through the control points (0, 0), (4, 0) and (8, 8) — its ends, and where their tangents
//! meet — which is exactly the parabola.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let [vertex, top, rim, control] =
        [(0.0, 0.0), (0.0, 8.0), (8.0, 8.0), (4.0, 0.0)].map(|(x, z)| section.point(zx_at(x, z)));
    let on_axis = section.line(vertex, top);
    section.line(top, rim);
    section.sketch.add_spline(vec![rim, control, vertex]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "paraboloid", "section", sketch_axis("section", on_axis), 360.0, Combine::NewBody)
}
