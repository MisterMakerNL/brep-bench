//! A plate from (−20, −20, 0) to (20, 20, 4) with a round boss on it, the cylinder of radius 5
//! on the z axis from z = 4 to z = 8. Both of the boss's circular edges are rounded to radius 2:
//! its rim (convex) and its root on the plate (concave), the two rounds meeting tangentially at
//! z = 6 so that the boss's side is gone and its profile is an S.
use crate::bench::{edge_through, frame, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(-20.0, -20.0, 0.0), p(20.0, 20.0, 4.0)).build(m)?;
    let boss = Cylinder::new(5.0, 4.0).with_frame(frame(p(0.0, 0.0, 4.0), Direction::Z)).build(m)?;
    let part = boolean::union(m, plate, boss)?;
    let rim = edge_through(m, part, p(0.0, 5.0, 8.0))?;
    let root = edge_through(m, part, p(0.0, 5.0, 4.0))?;
    Ok(blend::fillet(m, part, &[rim, root], 2.0)?)
}
