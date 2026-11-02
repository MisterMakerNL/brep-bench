//! A plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 through it on the z axis,
//! both of the hole's rims rounded to radius 1, half the plate's thickness: the two rounds meet
//! at the waist, z = 1, and the hole's wall is gone. The drill runs 1 past both faces.
use crate::bench::{edge_through, frame, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 2.0)).build(m)?;
    let drill = Cylinder::new(3.0, 4.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let part = boolean::difference(m, plate, drill)?;
    let top = edge_through(m, part, p(0.0, 3.0, 2.0))?;
    let bottom = edge_through(m, part, p(0.0, 3.0, 0.0))?;
    Ok(blend::fillet(m, part, &[top, bottom], 1.0)?)
}
