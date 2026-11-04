//! The part of the ball of radius 5 centred on the origin below the plane z = 4.5, shelled inward
//! to a wall 1 thick, open at its flat face: the inside, the sphere of radius 4, never reaches the
//! plane, so the flat face stays and the cavity is the closed ball of radius 4. The part is the
//! ball less a box above the plane.
use crate::shared::shell::ball_below_shelled_open;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    ball_below_shelled_open(4.5)
}
