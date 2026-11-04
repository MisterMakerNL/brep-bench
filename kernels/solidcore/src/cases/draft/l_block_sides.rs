//! An L-block: the outline through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25) and (0, 25),
//! extruded from z = 0 to z = 12; all six sides drafted by 6° about the neutral plane z = 0,
//! pull +z, each leaning into the material as it rises, the two at the concave edge too.
use crate::bench::polygon;
use crate::shared::draft::draft_about_z;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [30.0, 0.0, 0.0],
        [30.0, 10.0, 0.0],
        [10.0, 10.0, 0.0],
        [10.0, 25.0, 0.0],
        [0.0, 25.0, 0.0],
    ]);
    let block = extrude(&outline, [0.0, 0.0, 1.0], 12.0, 0.0, DraftCorner::default(), true)?;
    let sides = [
        [15.0, 0.0, 6.0],
        [30.0, 5.0, 6.0],
        [20.0, 10.0, 6.0],
        [10.0, 17.0, 6.0],
        [5.0, 25.0, 6.0],
        [0.0, 12.0, 6.0],
    ];
    draft_about_z(&block, &sides, 0.0, [0.0, 0.0, 1.0], 6.0)?;
    Ok(block)
}
