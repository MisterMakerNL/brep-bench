//! A plate from (−20, −20, 0) to (20, 20, 5) united with a cylindrical boss of radius 8 about
//! the z axis from z = 5 to z = 17. The boss's side is drafted 4° with the pull direction +z
//! about the neutral plane z = 5: a cone through its circle at z = 5, leaning inward as it rises,
//! the boss's top shrinking to meet it.
use crate::bench::{face_through, frame, p};
use crate::shared::draft::drafted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(-20.0, -20.0, 0.0), p(20.0, 20.0, 5.0)).build(m)?;
    let boss = Cylinder::new(8.0, 12.0).with_frame(frame(p(0.0, 0.0, 5.0), Direction::Z)).build(m)?;
    let part = boolean::union(m, plate, boss)?;
    let side = face_through(m, part, p(0.0, 8.0, 11.0))?;
    drafted(m, part, &[side], 4.0, p(0.0, 0.0, 5.0), Direction::Z)
}
