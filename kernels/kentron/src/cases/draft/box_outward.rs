//! The box from the origin to (20, 12, 10), its four sides drafted 5° the other way (a negative
//! draft) with the pull direction +z about the neutral plane z = 0, outward as they rise: an
//! upside-down frustum whose top grows to meet the drafted sides.
use crate::bench::p;
use crate::shared::draft::{box_sides, drafted};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (lo, hi) = (p(0.0, 0.0, 0.0), p(20.0, 12.0, 10.0));
    let block = Cuboid::from_corners(lo, hi).build(m)?;
    let sides = box_sides(m, block, lo, hi, 0.0)?;
    drafted(m, block, &sides, -5.0, p(0.0, 0.0, 0.0), Direction::Z)
}
