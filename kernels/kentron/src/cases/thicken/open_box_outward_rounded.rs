//! A sheet, the floor and four walls of the cube from the origin to (10, 10, 10) with no top,
//! thickened by 1 out of the cube, the gaps the moved faces leave at the tray's eight edges closed
//! by quarter cylinders of radius 1 about them, and at the floor's four corners by eighth balls.
use crate::shared::thicken::{thickened, tray};
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // the cube's faces' normals point out of it
    let sheet = tray(m)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
