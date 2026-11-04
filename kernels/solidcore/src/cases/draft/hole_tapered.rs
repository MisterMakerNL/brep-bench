//! The block from (−15, −15, 0) to (15, 15, 20), bored through along the z axis by a hole of
//! radius 6; the hole's wall drafted by 3° about the neutral plane z = 0, pull +z, leaning into
//! the material as it rises, so that the hole widens toward the top.
use crate::bench::cut;
use crate::shared::draft::draft_about_z;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-15.0, -15.0, 0.0], [30.0, 30.0, 20.0])?;
    // the bore runs past both faces
    let bore = make_cylinder([0.0, 0.0, -5.0], 6.0, 30.0)?;
    let bored = cut(block, bore)?;
    draft_about_z(&bored, &[[0.0, 6.0, 10.0]], 0.0, [0.0, 0.0, 1.0], 3.0)?;
    Ok(bored)
}
