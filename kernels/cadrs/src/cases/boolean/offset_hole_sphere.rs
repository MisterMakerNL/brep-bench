//! A ball of radius 20 centered on the origin, bored right through parallel to the y axis by a
//! hole of radius 6 about the line x = 5, z = 0: a circle extruded from y = −21 to y = 21, past
//! the ball at both ends, cut away. The ball is cadrs's sphere primitive.
use crate::bench::{ball, cut, cylinder, p3, V3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 20.0)?;
    let bore = cylinder(k, p3(5.0, -21.0, 0.0), V3::y(), 6.0, 42.0)?;
    cut(k, b, bore)
}
