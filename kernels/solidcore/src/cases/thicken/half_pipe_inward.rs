//! The half of the cylinder of radius 5 about the z axis where y >= 0, from z = 0 to z = 10, a
//! sheet, thickened by 2 toward the axis: the half tube between radii 3 and 5.
use crate::bench::arc;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{thicken, Body, Result};
use std::f64::consts::PI;

pub fn build() -> Result<Body> {
    // the half circle from (5, 0, 0) through (0, 5, 0) to (-5, 0, 0), swept along +z uncapped
    let half = arc([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 5.0, 0.0, PI);
    let sheet = extrude(&[half], [0.0, 0.0, 1.0], 10.0, 0.0, DraftCorner::default(), false)?;
    // its normal faces away from the axis: toward it is against the normal
    thicken(&sheet, -2.0, false)?;
    Ok(sheet)
}
