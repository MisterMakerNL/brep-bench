//! The rectangle in the xy plane from (0, 0, 0) to (10, 8, 0), a single flat face with no solid
//! behind it, thickened by 2 to its +z side: the box from the origin to (10, 8, 2).
use crate::bench::polygon;
use solidcore_ops::sweep::planar_faces;
use solidcore_ops::{thicken, Body, Result};

pub fn build() -> Result<Body> {
    // counter-clockwise seen from +z, so the face's normal is +z
    let sheet = planar_faces(&polygon(&[
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [10.0, 8.0, 0.0],
        [0.0, 8.0, 0.0],
    ]))?;
    thicken(&sheet, 2.0, false)?;
    Ok(sheet)
}
