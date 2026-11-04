//! The whole cylinder of radius 5 about the z axis from z = 0 to z = 10, a sheet bounded only by
//! its two circles, thickened by 1 away from the axis: the tube between radii 5 and 6.
use crate::bench::circle;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{thicken, Body, Result};

pub fn build() -> Result<Body> {
    // the circle swept along +z uncapped: the cylinder's one face
    let rim = circle([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 5.0);
    let sheet = extrude(&[rim], [0.0, 0.0, 1.0], 10.0, 0.0, DraftCorner::default(), false)?;
    // its normal faces away from the axis
    thicken(&sheet, 1.0, false)?;
    Ok(sheet)
}
