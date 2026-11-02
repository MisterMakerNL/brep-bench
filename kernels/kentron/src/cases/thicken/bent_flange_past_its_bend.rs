//! The bent sheet of `thicken/bent-flange` thickened by 3 to the inside of its bend, more than
//! the bend's radius of 2: the bend's moved face turns inside out and is no part of the solid,
//! and the floor moved up 3 and the wall moved in 3 meet in a concave edge at (−1, y, 3).
use crate::shared::thicken::{bent_sheet, thickened};
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // the faces' normals point out of the bend, so inside is a negative distance
    let sheet = bent_sheet(m)?;
    thickened(m, sheet, -3.0, Join::Arc)
}
