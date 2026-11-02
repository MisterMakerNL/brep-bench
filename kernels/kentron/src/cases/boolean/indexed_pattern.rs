//! A round disc of radius 20 on the z axis, from z = −5 to z = 0, with six round holes of radius 2
//! straight through it along z, their axes on the circle of radius 12: the first at (12, 0), the
//! others the first turned about the z axis by 60°, 120°, … 300°. Cut one after another, each
//! hole its own cut; each drill runs from z = −6 to z = 1, past both faces.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::{Axis, Direction, Transform};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let mut disc = Cylinder::new(20.0, 5.0).with_frame(frame(p(0.0, 0.0, -5.0), Direction::Z)).build(m)?;
    let first = Cylinder::new(2.0, 7.0).with_frame(frame(p(12.0, 0.0, -6.0), Direction::Z)).build(m)?;
    for k in 0..6 {
        let drill = first.moved(&Transform::rotation(Axis::Z, (60.0 * k as f64).to_radians()));
        disc = boolean::difference(m, disc, drill)?;
    }
    Ok(disc)
}
