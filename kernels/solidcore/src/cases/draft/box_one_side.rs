//! The box from the origin to (20, 12, 10), its side x = 20 drafted by 10° about the neutral
//! plane z = 0, pull +z: it turns about its bottom edge, leaning inward as it rises.
use crate::shared::draft::draft_about_z;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [20.0, 12.0, 10.0])?;
    draft_about_z(&block, &[[20.0, 6.0, 5.0]], 0.0, [0.0, 0.0, 1.0], 10.0)?;
    Ok(block)
}
