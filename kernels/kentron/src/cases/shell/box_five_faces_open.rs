//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at every face but
//! its bottom, z = 0. Only the bottom is left as a wall, moved 1 inward to z = 1 and running on to
//! the side openings' planes: the slab from the origin to (10, 8, 1).
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // open at the top and the four sides, each by a point in its middle
    let openings = [
        p(5.0, 4.0, 6.0),
        p(0.0, 4.0, 3.0),
        p(10.0, 4.0, 3.0),
        p(5.0, 0.0, 3.0),
        p(5.0, 8.0, 3.0),
    ];
    shell::inward(m, block, &openings, 1.0, Join::Arc)
}
