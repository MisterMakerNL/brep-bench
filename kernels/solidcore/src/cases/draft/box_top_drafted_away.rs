//! The box from the origin to (30, 10, 10), its four sides drafted by 30° about the neutral plane
//! z = 0, pull +z: the long sides meet at height 5√3, below the top, which the draft consumes,
//! leaving a hip roof with five faces.
use crate::shared::draft::{box_sides, draft_about_z};
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let size = [30.0, 10.0, 10.0];
    let block = make_box([0.0, 0.0, 0.0], size)?;
    draft_about_z(&block, &box_sides(size), 0.0, [0.0, 0.0, 1.0], 30.0)?;
    Ok(block)
}
