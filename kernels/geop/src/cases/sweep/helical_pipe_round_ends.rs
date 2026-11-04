//! A pipe of radius 1 along a helix about the z axis, of radius 10, rising 5 a turn, from
//! (10, 0, 0) three right-handed turns to (10, 0, 15), each end closed by a half ball, as a ball
//! rolled along the helix leaves it.
//!
//! geop sweeps a profile, not a solid, so the ball's sweep is built as what it is: the tube about
//! the ball's centre's path — the circle drawn square to the helix at its start and swept along
//! it — joined by a ball at each end, each turned from a half disc about a vertical line through
//! that end (geop has no ball of its own).
use crate::shared::paths::{at, path3d, plane_square_to, sketch3d, sweep};
use crate::shared::{Drawing, n, revolve, sketch, sketch_axis, zx, zx_at};
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
    sweep(b, "pipe", "section", path3d("helix"), Combine::NewBody)?;

    for (id, z) in [("first_end", 0.0), ("last_end", 15.0)] {
        let half = format!("{id}_half");
        let mut disc = Drawing::new();
        let below = disc.point(zx_at(10.0, z - 1.0));
        let above = disc.point(zx_at(10.0, z + 1.0));
        let on_axis = disc.line(below, above);
        disc.arc(above, below, 180.0);
        sketch(b, &half, zx(), disc)?;
        let pipe = b.solid()?;
        revolve(b, id, &half, sketch_axis(&half, on_axis), 360.0, Combine::join(pipe))?;
    }
    Ok(())
}
