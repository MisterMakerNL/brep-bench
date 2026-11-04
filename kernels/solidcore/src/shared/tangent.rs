//! What the solidcore implementations of the tangent cases share: the ball and the cylinder of
//! the Viviani cases, placed as those cases ask.
//!
//! solidcore's ball has its poles on its z axis and its seam the meridian on its +x side; its
//! cylinder has its seam the line on its +x side.
use crate::bench::{rotated, translated};
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

/// The ball of radius 10 centred on the origin with its poles on the y axis and its seam through
/// (-10, 0, 0): turned a half turn about z (the seam to -x), then a quarter turn about x (the
/// poles to y).
pub fn viviani_ball() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 10.0)?;
    let ball = rotated(ball, [0.0; 3], [0.0, 0.0, 1.0], 180.0)?;
    rotated(ball, [0.0; 3], [1.0, 0.0, 0.0], 90.0)
}

/// The cylinder of radius 5 on the line x = 5, y = 0, from z = -12 to z = 12, its seam on the z
/// axis, the side of its wall away from (10, 0, 0): turned a half turn about its axis, then
/// moved.
pub fn viviani_cylinder() -> Result<Body, Refusal> {
    let c = make_cylinder([0.0, 0.0, 0.0], 5.0, 24.0)?;
    let c = rotated(c, [0.0; 3], [0.0, 0.0, 1.0], 180.0)?;
    translated(c, [5.0, 0.0, -12.0])
}
