//! The part of the ball of radius 5 centred on the origin below the plane z = 3, shelled inward to
//! a wall 1 thick, open at its flat face. The sphere, moved 1 inward, is the sphere of radius 4,
//! which runs on to meet the plane in the circle of radius √7: the part of the ball of radius 5
//! below z = 3 less the part of the ball of radius 4 below it.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = shell::ball_below(m, 3.0)?;
    // open at the flat face, z = 3
    shell::inward(m, solid, &[p(0.0, 0.0, 3.0)], 1.0, Join::Arc)
}
