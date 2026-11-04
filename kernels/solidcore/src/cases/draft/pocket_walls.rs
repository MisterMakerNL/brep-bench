//! The block from the origin to (40, 30, 15) with a pocket cut into its top, the box from
//! (8, 8, 5) to (32, 22, 15); the pocket's four walls drafted by 5° about the neutral plane
//! z = 5, its floor, pull +z, each leaning into the material as it rises, so that the pocket
//! widens toward its opening.
use crate::bench::cut;
use crate::shared::draft::draft_about_z;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [40.0, 30.0, 15.0])?;
    // the pocket runs up past the block's top
    let pocket = make_box([8.0, 8.0, 5.0], [24.0, 14.0, 15.0])?;
    let part = cut(block, pocket)?;
    let walls = [[20.0, 8.0, 10.0], [32.0, 15.0, 10.0], [20.0, 22.0, 10.0], [8.0, 15.0, 10.0]];
    draft_about_z(&part, &walls, 5.0, [0.0, 0.0, 1.0], 5.0)?;
    Ok(part)
}
