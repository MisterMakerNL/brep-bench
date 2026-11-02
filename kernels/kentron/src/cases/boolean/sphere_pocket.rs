//! A block from (−10, −10, −10) to (10, 10, 0), with a ball of radius 5 centred on the origin cut
//! from it, leaving a hemispherical pocket in its top face.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::{Cuboid, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-10.0, -10.0, -10.0), p(10.0, 10.0, 0.0)).build(m)?;
    let ball = Sphere::new(5.0).build(m)?;
    boolean::difference(m, block, ball)
}
