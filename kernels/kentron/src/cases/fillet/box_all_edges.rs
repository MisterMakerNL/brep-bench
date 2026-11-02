//! A cube of side 10 with all twelve of its edges rounded to radius 1, three rounds meeting at
//! each corner.
use crate::bench::{edges, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let all = edges(m, cube);
    Ok(blend::fillet(m, cube, &all, 1.0)?)
}
