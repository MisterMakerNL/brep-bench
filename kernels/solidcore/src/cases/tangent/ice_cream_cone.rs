//! A ball of radius 15 centred on the origin, united with a solid cone on the z axis whose apex is
//! at (0, 0, -25) and whose base is the disc of radius 12 at z = -9, the cone's side tangent to
//! the sphere along the base's rim. solidcore's cones narrow upwards from their base, so the cone
//! is made with its base at z = -9 and its apex above, then turned over about the x axis
//! through the base's centre.
use crate::bench::{fuse, rotated};
use crate::Refusal;
use solidcore_ops::{make_cone, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 15.0)?;
    let cone = make_cone([0.0, 0.0, -9.0], 12.0, 0.0, 16.0)?;
    let cone = rotated(cone, [0.0, 0.0, -9.0], [1.0, 0.0, 0.0], 180.0)?;
    fuse(ball, cone)
}
