//! The lower half of the ball of radius 5 centred on the origin, shelled inward to a wall 1 thick,
//! open at its flat face, z = 0: a bowl, the half ball less the half ball of radius 4.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let half = shell::ball_below(m, 0.0)?;
    // open at the flat face, z = 0
    shell::inward(m, half, &[p(0.0, 0.0, 0.0)], 1.0, Join::Arc)
}
