//! A cube of side 10 between the origin and (10, 10, 10), with a saw cut a millionth wide down
//! into it from the top: the slot is the block x = 5 … 5.000001, right through in y, from z = 5 up
//! through the top. The saw is a block that runs past the cube's front, back and top.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let saw = Cuboid::from_corners(p(5.0, -1.0, 5.0), p(5.000001, 11.0, 11.0)).build(m)?;
    boolean::difference(m, cube, saw)
}
