//! A plate from (−20, −20, 0) to (20, 20, 5) with a boss on it, the cylinder of radius 8 on the
//! z axis from z = 5 to z = 17, united; the boss's side drafted by 4° about the neutral plane
//! z = 5, pull +z: a cone leaning inward as it rises, the boss's top shrinking to meet it.
use crate::bench::fuse;
use crate::shared::draft::draft_about_z;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([-20.0, -20.0, 0.0], [40.0, 40.0, 5.0])?;
    let boss = make_cylinder([0.0, 0.0, 5.0], 8.0, 12.0)?;
    let part = fuse(plate, boss)?;
    draft_about_z(&part, &[[0.0, 8.0, 11.0]], 5.0, [0.0, 0.0, 1.0], 4.0)?;
    Ok(part)
}
