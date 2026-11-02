//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 3 thick, open at its top face, z = 10. A round moved 3 inward would be
//! turned inside out: it is no part of the result, and the inside walls on either side run on
//! until they meet in a sharp edge, so the cavity is the box from (3, 3, 3) to (17, 11, 10).
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = shell::rounded_plate(m)?;
    // open at the top, z = 10
    shell::inward(m, plate, &[p(10.0, 7.0, 10.0)], 3.0, Join::Arc)
}
