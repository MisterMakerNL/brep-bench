//! A pipe of radius 1 along a helix about the z axis, of radius 10, rising 5 a turn, from
//! (10, 0, 0) three right-handed turns: the helix drawn in a 3-D sketch, the circle drawn on the
//! plane square to it at its start — through (10, 0, 0), square to its tangent there,
//! (0, 2π·10, 5) — and swept along it.
use crate::shared::paths::{at, path3d, plane_square_to, sketch3d, sweep};
use crate::shared::{Drawing, n, sketch};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;
use geop_ops_model::sketch3d::Sketch3d;
use std::f64::consts::PI;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut route = Sketch3d::new();
    let start = route.add_point(at([10.0, 0.0, 0.0]));
    let base = route.add_point(at([0.0, 0.0, 0.0]));
    let top = route.add_point(at([0.0, 0.0, 15.0]));
    route.add_helix(start, base, top, n(3.0), false)?;
    sketch3d(b, "helix", route)?;

    let across = plane_square_to(b, "across", [10.0, 0.0, 0.0], [0.0, 2.0 * PI * 10.0, 5.0])?;
    let mut section = Drawing::new();
    section.circle([0.0, 0.0], 1.0);
    sketch(b, "section", across, section)?;
    sweep(b, "pipe", "section", path3d("helix"), Combine::NewBody)
}
