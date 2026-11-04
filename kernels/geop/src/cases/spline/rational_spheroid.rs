//! A prolate spheroid 12 wide and 20 tall, turned a full turn about the z axis from a region of
//! the xz plane: the z axis from (0, 0, −10) to (0, 0, 10), and the rational quadratic B-spline
//! joining them through the control points (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10),
//! (0, 0, 10), weighted 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ½, ½, 1, 1, 1) — a sketch
//! spline given that shape — turned about its straight side.
use crate::shared::{Drawing, n, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_sketch::{CurveKind, SplineShape};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let w = 0.5f64.sqrt();
    let mut region = Drawing::new();
    let points = [(0.0, -10.0), (6.0, -10.0), (6.0, 0.0), (6.0, 10.0), (0.0, 10.0)]
        .map(|(x, z)| region.point(zx_at(x, z)));
    region.sketch.add_curve(CurveKind::Spline {
        control_points: points.to_vec(),
        shape: Some(SplineShape {
            degree: 2,
            knots: [0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0].map(n).to_vec(),
            weights: [1.0, w, 1.0, w, 1.0].map(n).to_vec(),
        }),
    });
    let on_axis = region.line(points[4], points[0]);
    sketch(b, "region", zx(), region)?;
    revolve(b, "spheroid", "region", sketch_axis("region", on_axis), 360.0, Combine::NewBody)
}
