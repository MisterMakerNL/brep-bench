//! The part of the ball of radius 5 centred on the origin below the plane z = 3.9999, shelled
//! inward to a wall 1 thick, open at its flat face: the inside, the sphere of radius 4, meets the
//! plane in a pinhole of radius about 0.028, through which the cavity opens. The part is the ball
//! less a box above the plane.
use crate::shared::shell::ball_below_shelled_open;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    ball_below_shelled_open(3.9999)
}
