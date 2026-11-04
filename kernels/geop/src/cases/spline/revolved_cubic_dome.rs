//! A dome turned a full turn about the z axis from a region of the xz plane: the z axis from the
//! origin to (0, 0, 10), the x axis out to (9, 0, 0), and the cubic Bézier curve with control
//! points (9, 0, 0), (12, 0, 4), (9, 0, 10) and (0, 0, 10) — a sketch spline of four control
//! points, which is that Bézier curve — turned about its straight side on the z axis.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut region = Drawing::new();
    let origin = region.point(zx_at(0.0, 0.0));
    let rim = region.point(zx_at(9.0, 0.0));
    let pulls = [region.point(zx_at(12.0, 4.0)), region.point(zx_at(9.0, 10.0))];
    let top = region.point(zx_at(0.0, 10.0));
    region.line(origin, rim);
    region.sketch.add_spline(vec![rim, pulls[0], pulls[1], top]);
    let on_axis = region.line(top, origin);
    sketch(b, "region", zx(), region)?;
    revolve(b, "dome", "region", sketch_axis("region", on_axis), 360.0, Combine::NewBody)
}
