//! A rod, the cylinder of radius 5 about the z axis from z = 0 to 20, with a helical groove cut
//! round it by a ball of radius 1 whose centre runs along the helix of radius 5 about the z axis
//! from (5, 0, −2), right-handed, rising 4 a turn, six turns to (5, 0, 22).
//!
//! geop sweeps a profile, not a solid, so what the ball passes through is built as what it is:
//! the tube of radius 1 about its centre's path — the helix drawn in a 3-D sketch, the circle on
//! the plane square to it at its start, through (5, 0, −2) square to its tangent (0, 2π·5, 4),
//! swept along it — and a ball at each end; the balls at the ends, below z = −1 and above z = 21,
//! are clear of the rod, so only the tube is cut from it.
use crate::shared::paths::{at, path3d, plane_square_to, sketch3d, sweep};
use crate::shared::{Drawing, extrude, n, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;
use geop_ops_model::sketch3d::Sketch3d;
use std::f64::consts::PI;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut end = Drawing::new();
    end.circle([0.0, 0.0], 5.0);
    sketch(b, "end", xy(), end)?;
    extrude(b, "rod", "end", 20.0, Combine::NewBody)?;

    let mut route = Sketch3d::new();
    let start = route.add_point(at([5.0, 0.0, -2.0]));
    let base = route.add_point(at([0.0, 0.0, -2.0]));
    let top = route.add_point(at([0.0, 0.0, 22.0]));
    route.add_helix(start, base, top, n(6.0), false)?;
    sketch3d(b, "helix", route)?;

    let across = plane_square_to(b, "across", [5.0, 0.0, -2.0], [0.0, 2.0 * PI * 5.0, 4.0])?;
    let mut section = Drawing::new();
    section.circle([0.0, 0.0], 1.0);
    sketch(b, "section", across, section)?;
    let rod = b.solid()?;
    sweep(b, "groove", "section", path3d("helix"), Combine::cut(rod))
}
