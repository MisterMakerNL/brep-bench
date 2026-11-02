//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top face,
//! z = 6, its edges kept sharp: the box from (-1, -1, -1) to (11, 9, 6) less the box itself.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // outward, joined by intersection: the moved faces extended until they meet
    shell::outward(m, block, &[p(5.0, 4.0, 6.0)], 1.0, Join::Intersection)
}
