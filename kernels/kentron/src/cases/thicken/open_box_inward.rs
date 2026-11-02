//! A sheet, the floor and four walls of the cube from the origin to (10, 10, 10) with no top,
//! thickened by 1 into the cube: the moved faces cross near the tray's edges and are trimmed
//! where they meet, the cube less the box from (1, 1, 1) to (9, 9, 10).
use crate::shared::thicken::{thickened, tray};
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // the cube's faces' normals point out of it, so into it is a negative distance
    let sheet = tray(m)?;
    thickened(m, sheet, -1.0, Join::Arc)
}
