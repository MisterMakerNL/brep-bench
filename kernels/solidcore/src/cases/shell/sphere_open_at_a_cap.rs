//! The part of the ball of radius 5 centred on the origin below the plane z = 3, shelled inward to
//! a wall 1 thick, open at its flat face: the part of the ball below z = 3 less the part of the
//! ball of radius 4 below it, its rim the annulus between radii √7 and 4. The part is the ball
//! less a box above the plane.
use crate::shared::shell::ball_below_shelled_open;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    ball_below_shelled_open(3.0)
}
