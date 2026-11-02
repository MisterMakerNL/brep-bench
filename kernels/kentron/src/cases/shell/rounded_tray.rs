//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 1 thick, open at its top face, z = 10. Each round, moved 1 inward, is
//! the cylinder of radius 1 on its axis: a tray whose inside corners are concentric with its
//! outside ones.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = shell::rounded_plate(m)?;
    // open at the top, z = 10
    shell::inward(m, plate, &[p(10.0, 7.0, 10.0)], 1.0, Join::Arc)
}
