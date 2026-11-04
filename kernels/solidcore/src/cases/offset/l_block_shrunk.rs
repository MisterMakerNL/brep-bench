//! The L-shaped plate (the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) shrunk by 1: every point at least 1 from its outside, its convex edges
//! sharp and its concave edge a concave quarter cylinder, the kernel's offset inward, rounded
//! where the moved faces part.
use crate::bench::polygon;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [20.0, 0.0, 0.0],
        [20.0, 5.0, 0.0],
        [5.0, 5.0, 0.0],
        [5.0, 15.0, 0.0],
        [0.0, 15.0, 0.0],
    ]);
    let plate = extrude(&outline, [0.0, 0.0, 1.0], 8.0, 0.0, DraftCorner::default(), true)?;
    offset_rounded(&plate, -1.0)
}
