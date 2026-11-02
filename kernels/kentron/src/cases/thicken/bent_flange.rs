//! A bent sheet, a flat floor, a bend of radius 2 about the line through (0, ·, 2) along y and a
//! flat upright wall, meeting tangentially, 8 wide in y, thickened by 1 to the outside of the
//! bend (below the floor, beyond the wall): a bracket 1 thick, its bend's radii 2 and 3.
use crate::shared::thicken::{bent_sheet, thickened};
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // the profile runs from the floor to the wall; swept along +y, its faces' normals point out
    // of the bend
    let sheet = bent_sheet(m)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
