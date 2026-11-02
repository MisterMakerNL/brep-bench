//! The box from the origin to (30, 20, 20), drilled right through along x by a hole of radius 4
//! about the line y = 10, z = 10 (the drill running 1 past both ends). Then the two faces the
//! hole comes out of, x = 0 and x = 30, are drafted 8° with the pull direction +z about the
//! neutral plane z = 0, inward as they rise; the hole's wall stays the cylinder, trimmed by them.
use crate::bench::{face_through, frame, p};
use crate::shared::draft::drafted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(30.0, 20.0, 20.0)).build(m)?;
    let drill = Cylinder::new(4.0, 32.0).with_frame(frame(p(-1.0, 10.0, 10.0), Direction::X)).build(m)?;
    let part = boolean::difference(m, block, drill)?;
    let near = face_through(m, part, p(0.0, 2.0, 2.0))?;
    let far = face_through(m, part, p(30.0, 2.0, 2.0))?;
    drafted(m, part, &[near, far], 8.0, p(0.0, 0.0, 0.0), Direction::Z)
}
