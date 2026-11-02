//! A rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin and the
//! opposite corner at (10, 20, 30).
use crate::bench::p;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 20.0, 30.0)).build(m)
}
