//! Two cubes of side 10 side by side, between the origin and (10, 10, 10) and between (10, 0, 0)
//! and (20, 10, 10), joined into one solid: the whole face they share disappears.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let second = Cuboid::from_corners(p(10.0, 0.0, 0.0), p(20.0, 10.0, 10.0)).build(m)?;
    boolean::union(m, first, second)
}
