//! The box from the origin to (10, 8, 6), shrunk by 1: every point at least 1 from its outside,
//! the box from (1, 1, 1) to (9, 7, 5).
use crate::bench::p;
use kentron::offset::{self, Join};
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    offset::shape(m, block, -1.0, Join::Arc)
}
