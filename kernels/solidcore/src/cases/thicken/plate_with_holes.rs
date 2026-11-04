//! The square in the xy plane from (-10, -10, 0) to (10, 10, 0) with two round holes of radius 3
//! centred on (-5, 0, 0) and (5, 0, 0), one flat face, thickened by 1 to each side: the plate
//! from z = -1 to z = 1, holed through by two cylinders.
use crate::bench::{circle, polygon};
use solidcore_ops::sweep::planar_faces;
use solidcore_ops::{thicken, Body, Result};

pub fn build() -> Result<Body> {
    let mut loops =
        polygon(&[[-10.0, -10.0, 0.0], [10.0, -10.0, 0.0], [10.0, 10.0, 0.0], [-10.0, 10.0, 0.0]]);
    // the holes run the other way round
    for x in [-5.0, 5.0] {
        loops.push(circle([x, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0], 3.0));
    }
    let sheet = planar_faces(&loops)?;
    // 2 thick in all, moved back by half first
    thicken(&sheet, 2.0, true)?;
    Ok(sheet)
}
