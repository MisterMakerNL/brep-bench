//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top face,
//! z = 6: the wall is every point outside the box within 1 of its other faces, up to the plane of
//! the top, its outside rounded by a quarter cylinder about each convex edge and an eighth of a
//! ball at each bottom corner.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // outward, joined by arcs: the moved faces that part at the box's edges joined by rounds
    shell::outward(m, block, &[p(5.0, 4.0, 6.0)], 1.0, Join::Arc)
}
