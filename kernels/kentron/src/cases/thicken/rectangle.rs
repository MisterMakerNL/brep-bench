//! A sheet, the rectangle in the xy plane from (0, 0, 0) to (10, 8, 0), thickened by 2 to its +z
//! side: the box from the origin to (10, 8, 2).
use crate::bench::p;
use crate::shared::offset::polygon_face;
use crate::shared::thicken::thickened;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // anticlockwise seen from +z: its normal is +z
    let sheet = polygon_face(m, &[p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 8.0, 0.0), p(0.0, 8.0, 0.0)])?;
    thickened(m, sheet, 2.0, Join::Arc)
}
