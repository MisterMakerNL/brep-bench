//! Two cubes of side 10 joined into one solid: the first between the origin and (10, 10, 10), the
//! second between (5, 0, 10) and (15, 10, 20), standing on the first over half of its top.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let second = Cuboid::from_corners(p(5.0, 0.0, 10.0), p(15.0, 10.0, 20.0)).build(m)?;
    boolean::union(m, first, second)
}
