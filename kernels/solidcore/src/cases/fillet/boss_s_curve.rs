//! A plate from (−20, −20, 0) to (20, 20, 4) with a boss on it, the cylinder of radius 5 on the
//! z axis from z = 4 to z = 8; both circular edges of the boss rounded to radius 2, its rim at
//! z = 8 (convex) and its root at z = 4 (concave), so the rounds meet at z = 6 and the boss's
//! side vanishes.
use crate::bench::{edges_through_each, fuse, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_box, make_cylinder, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([-20.0, -20.0, 0.0], [40.0, 40.0, 4.0])?;
    let boss = make_cylinder([0.0, 0.0, 4.0], 5.0, 4.0)?;
    let (body, order) = reloaded(&fuse(plate, boss)?)?;
    let rims = edges_through_each(&body, &order, &[[0.0, 5.0, 8.0], [0.0, 5.0, 4.0]])?;
    blend(&body, Blend::Round, &rims, &[2.0], &[], Corners::Filled)?;
    Ok(body)
}
