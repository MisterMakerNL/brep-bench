//! The box from the origin to (30, 20, 10) with its four vertical edges rounded to radius 4, its
//! eight sides (four planes, four quarter cylinders) drafted 5° with the pull direction +z about
//! the neutral plane z = 0, inward as they rise: each round becomes a quarter cone about the same
//! axis, tangent to the drafted planes beside it, and the top is trimmed to them.
use crate::bench::p;
use crate::shared::draft::{box_sides, drafted, rounded_box};
use crate::Refusal;
use kentron::math::Direction;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (lo, hi) = (p(0.0, 0.0, 0.0), p(30.0, 20.0, 10.0));
    let block = rounded_box(m, lo, hi, 4.0)?;
    let sides = box_sides(m, block, lo, hi, 4.0)?;
    drafted(m, block, &sides, 5.0, p(0.0, 0.0, 0.0), Direction::Z)
}
