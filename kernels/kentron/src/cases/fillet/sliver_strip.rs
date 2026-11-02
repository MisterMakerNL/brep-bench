//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 that run along x
//! each rounded to radius 0.9999, leaving a strip of that face 0.0002 tall between the rounds.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 2.0)).build(m)?;
    let bottom = edge_through(m, plate, p(10.0, 10.0, 0.0))?;
    let top = edge_through(m, plate, p(10.0, 10.0, 2.0))?;
    Ok(blend::fillet(m, plate, &[bottom, top], 0.9999)?)
}
