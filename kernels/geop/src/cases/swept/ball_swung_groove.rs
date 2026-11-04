//! A block, the box from (−6, −6, −8) to (18, 18, 0), with a groove cut into its top by a ball of
//! radius 3 whose centre swings a quarter turn about the z axis in the top face, from (12, 0, 0)
//! to (0, 12, 0).
//!
//! geop sweeps a profile, not a solid, so what the ball passes through is built as what it is:
//! the tube about its centre's path — the ball's great circle on the xz plane, about (12, 0, 0),
//! turned a quarter turn about the z axis — and the ball at each end (turned from a half disc).
//! Each is cut from the block.
use crate::shared::paths::ball;
use crate::shared::{Drawing, axis, extrude, revolve, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-6.0, -6.0], [18.0, 18.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", -8.0, Combine::NewBody)?;

    let mut section = Drawing::new();
    section.circle(zx_at(12.0, 0.0), 3.0);
    sketch(b, "section", zx(), section)?;
    let block = b.solid()?;
    // Turned from +x towards +y: −90° as geop's revolve counts it for a profile on the xz plane.
    revolve(b, "groove", "section", axis(FrameAxis::Z), -90.0, Combine::cut(block))?;

    for (id, centre) in [("first_end", [12.0, 0.0, 0.0]), ("last_end", [0.0, 12.0, 0.0])] {
        let block = b.solid()?;
        ball(b, id, centre, 3.0, Combine::cut(block))?;
    }
    Ok(())
}
