//! A plate from the origin to (100, 20, 10), drilled through along z by nine holes of radius 2 at
//! y = 10 and x = 10, 20, … 90, one after another, each hole its own cut. Each drill runs from
//! z = −1 to z = 11, past both faces.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let mut plate = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(100.0, 20.0, 10.0)).build(m)?;
    for k in 1..=9 {
        let x = 10.0 * k as f64;
        let drill = Cylinder::new(2.0, 12.0).with_frame(frame(p(x, 10.0, -1.0), Direction::Z)).build(m)?;
        plate = boolean::difference(m, plate, drill)?;
    }
    Ok(plate)
}
