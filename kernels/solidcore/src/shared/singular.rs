//! What the solidcore implementations of the singular cases share: a half-space as a block
//! reaching past the solid it cuts, tilted to the cutting plane.
use crate::bench::rotated;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

/// The block on the side x > `x` of the plane x = `x`, reaching 30 past the line through
/// (`x`, 0, 0) along y every other way, then turned `degrees` about that line: a half-space
/// through it, for the solids of these cases, which lie within 10 of the origin.
pub fn beyond(x: f64, degrees: f64) -> Result<Body, Refusal> {
    let block = make_box([x, -30.0, -30.0], [30.0, 60.0, 60.0])?;
    rotated(block, [x, 0.0, 0.0], [0.0, 1.0, 0.0], degrees)
}
