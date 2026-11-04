//! A block from (-10, -10, 0) to (10, 10, 10), united with a solid cone on the z axis standing on
//! its tip at the origin, its base the disc of radius 5 in the plane z = 15. solidcore's cones
//! narrow upwards from their base, so the cone is made with its base on the xy plane and its
//! apex at z = 15, then turned over about the x axis at its middle height.
use crate::bench::{fuse, rotated};
use crate::Refusal;
use solidcore_ops::{make_box, make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-10.0, -10.0, 0.0], [20.0, 20.0, 10.0])?;
    let cone = make_cone([0.0, 0.0, 0.0], 5.0, 0.0, 15.0)?;
    let cone = rotated(cone, [0.0, 0.0, 7.5], [1.0, 0.0, 0.0], 180.0)?;
    fuse(block, cone)
}
