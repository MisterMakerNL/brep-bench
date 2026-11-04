//! A pipe of radius 20 along the planar cubic Bézier curve with control points (0, 0, 0),
//! (12, −16, 0), (32, −16, 0) and (44, 0, 0): the curve sketched on the xy plane as a spline of
//! those four control points, the circle drawn on the plane square to it at its start — through
//! the origin, square to its tangent there, (3, −4, 0) — and swept along it.
use crate::shared::paths::{path2d, plane_square_to, sweep};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut path = Drawing::new();
    let points = [[0.0, 0.0], [12.0, -16.0], [32.0, -16.0], [44.0, 0.0]].map(|p| path.point(p));
    path.sketch.add_spline(points.to_vec());
    sketch(b, "path", xy(), path)?;

    let across = plane_square_to(b, "across", [0.0, 0.0, 0.0], [3.0, -4.0, 0.0])?;
    let mut section = Drawing::new();
    section.circle([0.0, 0.0], 20.0);
    sketch(b, "section", across, section)?;
    sweep(b, "pipe", "section", path2d("path"), Combine::NewBody)
}
