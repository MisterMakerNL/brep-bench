//! A cap of a ball, turned a full turn about the z axis from a region of the xz plane: the z axis
//! from (0, 0, −5) to (0, 0, 10), the segment out to (5√3, 0, −5), and the rational quadratic
//! Bézier curve from there to the top, its control points (5√3, 0, −5), (10√3, 0, 10), (0, 0, 10)
//! weighted 1, ½, 1 — a sketch spline given that shape — turned about its straight side.
use crate::shared::{Drawing, n, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_sketch::{CurveKind, SplineShape};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let r3 = 3f64.sqrt();
    let mut region = Drawing::new();
    let bottom = region.point(zx_at(0.0, -5.0));
    let rim = region.point(zx_at(5.0 * r3, -5.0));
    let pull = region.point(zx_at(10.0 * r3, 10.0));
    let top = region.point(zx_at(0.0, 10.0));
    region.line(bottom, rim);
    region.sketch.add_curve(CurveKind::Spline {
        control_points: vec![rim, pull, top],
        shape: Some(SplineShape {
            degree: 2,
            knots: [0.0, 0.0, 0.0, 1.0, 1.0, 1.0].map(n).to_vec(),
            weights: [1.0, 0.5, 1.0].map(n).to_vec(),
        }),
    });
    let on_axis = region.line(top, bottom);
    sketch(b, "region", zx(), region)?;
    revolve(b, "cap", "region", sketch_axis("region", on_axis), 360.0, Combine::NewBody)
}
