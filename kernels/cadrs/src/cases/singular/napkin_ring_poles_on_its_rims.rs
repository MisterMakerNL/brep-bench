//! A ball of radius 5 centered on the origin with a hole of radius 3 bored right through it along
//! the z axis: the napkin ring of `boolean/napkin-ring`, its rims the circles of radius 3 at
//! z = ±4. The ball's poles are on the line through the origin along (3, 0, 4): at (3, 0, 4), on
//! the upper rim, and at (−3, 0, −4), on the lower one.
//!
//! The ball is the half disc beside that line, sketched on a plane through it, revolved a whole
//! turn about it (cadrs has no sphere primitive). The hole is cut with a cylinder of radius 3
//! about the z axis from z = −6 to z = 6, past both ends of the ball.
use crate::bench::{cut, cylinder, p3, v3};
use crate::shared::singular::ball_about;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball_about(k, p3(0.0, 0.0, 0.0), v3(3.0, 0.0, 4.0), 5.0)?;
    let hole = cylinder(k, p3(0.0, 0.0, -6.0), v3(0.0, 0.0, 1.0), 3.0, 12.0)?;
    cut(k, b, hole)
}
