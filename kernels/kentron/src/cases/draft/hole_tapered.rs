//! The block from (−15, −15, 0) to (15, 15, 20), bored right through along the z axis by a hole
//! of radius 6 (the drill running 1 past both faces). The hole's wall is drafted 3° with the pull
//! direction +z about the neutral plane z = 0: a cone through its circle at z = 0, widening as it
//! rises.
use crate::bench::{face_through, frame, p};
use crate::shared::draft::drafted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(-15.0, -15.0, 0.0), p(15.0, 15.0, 20.0)).build(m)?;
    let drill = Cylinder::new(6.0, 22.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let part = boolean::difference(m, block, drill)?;
    let wall = face_through(m, part, p(0.0, 6.0, 10.0))?;
    // (A hole's wall faces the axis: a positive draft leans it toward the material as it rises.)
    drafted(m, part, &[wall], 3.0, p(0.0, 0.0, 0.0), Direction::Z)
}
