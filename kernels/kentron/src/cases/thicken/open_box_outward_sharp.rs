//! A sheet, the floor and four walls of the cube from the origin to (10, 10, 10) with no top,
//! thickened by 1 out of the cube, the moved faces extended until they meet: the box from
//! (−1, −1, −1) to (11, 11, 10) less the cube.
use crate::shared::thicken::{thickened, tray};
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // the cube's faces' normals point out of it
    let sheet = tray(m)?;
    thickened(m, sheet, 1.0, Join::Intersection)
}
