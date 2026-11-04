//! A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top by a ball of
//! radius 3 whose centre runs along the cubic Bézier curve in the top face with control points
//! (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0).
//!
//! geop sweeps a profile, not a solid, so what the ball passes through is built as what it is:
//! the tube of radius 3 about its centre's path — the curve sketched on the xy plane as a spline of
//! those control points, the circle on the yz plane, square to it at its start, swept along it —
//! and the ball at each end of the path (turned from a half disc). Each is cut from the block.
use crate::shared::paths::{ball, path2d, sweep};
use crate::shared::{Drawing, extrude, sketch, xy, yz};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-6.0, -6.0], [36.0, 26.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", -8.0, Combine::NewBody)?;

    let mut path = Drawing::new();
    let points = [[0.0, 0.0], [10.0, 0.0], [20.0, 20.0], [30.0, 20.0]].map(|p| path.point(p));
    path.sketch.add_spline(points.to_vec());
    sketch(b, "path", xy(), path)?;
    let mut section = Drawing::new();
    section.circle([0.0, 0.0], 3.0);
    sketch(b, "section", yz(), section)?;
    let block = b.solid()?;
    sweep(b, "groove", "section", path2d("path"), Combine::cut(block))?;

    for (id, centre) in [("first_end", [0.0, 0.0, 0.0]), ("last_end", [30.0, 20.0, 0.0])] {
        let block = b.solid()?;
        ball(b, id, centre, 3.0, Combine::cut(block))?;
    }
    Ok(())
}
