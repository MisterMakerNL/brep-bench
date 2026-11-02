//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 2 thick, open at its top face, z = 10. Each round, moved 2 inward,
//! shrinks to its axis, where the inside walls on either side meet in a sharp edge: the cavity is
//! the sharp-cornered box from (2, 2, 2) to (18, 12, 10).
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = shell::rounded_plate(m)?;
    // open at the top, z = 10
    shell::inward(m, plate, &[p(10.0, 7.0, 10.0)], 2.0, Join::Arc)
}
