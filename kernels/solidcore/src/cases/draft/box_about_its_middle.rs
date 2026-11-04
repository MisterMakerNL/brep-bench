//! The box from the origin to (20, 12, 10), its four sides drafted by 5° about the neutral plane
//! z = 5, halfway up, pull +z: each moves out below z = 5 and in above it.
use crate::shared::draft::{box_sides, draft_about_z};
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let size = [20.0, 12.0, 10.0];
    let block = make_box([0.0, 0.0, 0.0], size)?;
    draft_about_z(&block, &box_sides(size), 5.0, [0.0, 0.0, 1.0], 5.0)?;
    Ok(block)
}
