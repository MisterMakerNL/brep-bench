//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x each
//! chamfered with distance 2 on both faces, half the top's width: the chamfers meet at right
//! angles along the ridge y = 2, z = 6 and the top face is gone.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 4.0, 6.0)).build(m)?;
    let near = edge_through(m, block, p(5.0, 0.0, 6.0))?;
    let far = edge_through(m, block, p(5.0, 4.0, 6.0))?;
    Ok(blend::chamfer(m, block, &[near, far], 2.0)?)
}
