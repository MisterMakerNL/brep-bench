//! A block from (−10, −10, 0) to (10, 10, 10), united with a solid cone on the z axis whose apex
//! is at the origin, the middle of the block's bottom face, and whose base is the disc of radius 5
//! at z = 15. The tip touches the bottom face from inside at one point; above z = 10 the cone
//! stands out of the top as a frustum.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::{Cone, Cuboid};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    // radius 0 at its frame's origin, the apex, and 5 at its top, 15 above
    let cone = Cone::new(0.0, 5.0, 15.0).build(m)?;
    boolean::union(m, block, cone)
}
