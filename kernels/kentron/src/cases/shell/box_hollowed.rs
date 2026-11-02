//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick with no opening: the box less
//! the box from (1, 1, 1) to (9, 7, 5), one solid bounded by two shells, the cavity's facing in.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 8.0, 6.0)).build(m)?;
    // thick_solid given no opening: the moved faces bound a closed cavity of their own
    shell::hollowed(m, block, 1.0)
}
