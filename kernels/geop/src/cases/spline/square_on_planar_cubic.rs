//! A bar of square section 20 × 20 along the planar cubic Bézier curve with control points
//! (0, 0, 0), (16, 12, 0), (18, 26, 0) and (12, 34, 0), two of its sides horizontal: the curve
//! sketched on the xy plane as a spline of those four control points, the square drawn on the
//! plane square to it at its start — through the origin, square to its tangent there, (4, 3, 0),
//! its first axis z, so the square's sides drawn along the axes are horizontal and upright — and
//! swept along it.
use crate::shared::paths::{path2d, plane_square_to, sweep};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut path = Drawing::new();
    let points = [[0.0, 0.0], [16.0, 12.0], [18.0, 26.0], [12.0, 34.0]].map(|p| path.point(p));
    path.sketch.add_spline(points.to_vec());
    sketch(b, "path", xy(), path)?;

    let across = plane_square_to(b, "across", [0.0, 0.0, 0.0], [4.0, 3.0, 0.0])?;
    let mut section = Drawing::new();
    section.rectangle([-10.0, -10.0], [10.0, 10.0]);
    sketch(b, "section", across, section)?;
    sweep(b, "bar", "section", path2d("path"), Combine::NewBody)
}
