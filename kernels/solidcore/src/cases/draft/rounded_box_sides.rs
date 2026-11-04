//! The box from the origin to (30, 20, 10) with its four vertical edges rounded to radius 4; all
//! eight sides, four planes and four quarter cylinders, drafted by 5° about the neutral plane
//! z = 0, pull +z, each leaning inward as it rises, the rounds becoming quarter cones.
use crate::shared::draft::{draft_about_z, rounded_box, rounded_sides};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let block = rounded_box([0.0, 0.0, 0.0], [30.0, 20.0, 10.0], 4.0)?;
    let sides = rounded_sides(0.0, 0.0, 30.0, 20.0, 4.0, 5.0);
    draft_about_z(&block, &sides, 0.0, [0.0, 0.0, 1.0], 5.0)?;
    Ok(block)
}
