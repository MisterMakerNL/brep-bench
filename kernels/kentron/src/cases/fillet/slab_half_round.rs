//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x each
//! rounded to radius 2, half the top's width: the rounds meet along y = 2, z = 6 and the top is
//! a half-cylinder.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 4.0, 6.0)).build(m)?;
    let near = edge_through(m, block, p(5.0, 0.0, 6.0))?;
    let far = edge_through(m, block, p(5.0, 4.0, 6.0))?;
    Ok(blend::fillet(m, block, &[near, far], 2.0)?)
}
