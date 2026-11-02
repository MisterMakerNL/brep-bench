//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded to radius 2.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let edge = edge_through(m, cube, p(10.0, 10.0, 5.0))?;
    Ok(blend::fillet(m, cube, &[edge], 2.0)?)
}
