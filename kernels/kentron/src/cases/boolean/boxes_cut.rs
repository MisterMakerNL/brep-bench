//! Two blocks 10 high standing on the xy plane, the first spanning x and y from 0 to 10, the
//! second from 5 to 15: the second cut away from the first.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let second = Cuboid::from_corners(p(5.0, 5.0, 0.0), p(15.0, 15.0, 10.0)).build(m)?;
    boolean::difference(m, first, second)
}
