//! The part of the ball of radius 5 centred on the origin below the plane z = 4.5, shelled inward
//! to a wall 1 thick, open at its flat face. The sphere, moved 1 inward, is the sphere of radius 4,
//! wholly below the plane: the cavity never reaches the opening, the flat face stays, and the
//! result is the part of the ball below z = 4.5 less the closed ball of radius 4.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = shell::ball_below(m, 4.5)?;
    // open at the flat face, z = 4.5
    shell::inward(m, solid, &[p(0.0, 0.0, 4.5)], 1.0, Join::Arc)
}
