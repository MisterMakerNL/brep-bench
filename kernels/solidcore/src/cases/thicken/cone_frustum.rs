//! The side of the cone frustum about the z axis whose radius runs from 6 at z = 0 to 3 at
//! z = 4, one face bounded by those circles, thickened by 1 along its normal away from the axis:
//! its rims sweep frusta of cones.
use crate::bench::line;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{thicken, Body, Result};
use std::f64::consts::TAU;

pub fn build() -> Result<Body> {
    // the slant line, turned a full turn about the z axis uncapped
    let slant = line([6.0, 0.0, 0.0], [3.0, 0.0, 4.0]);
    let sheet = revolve(&[slant], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    // its normal faces away from the axis
    thicken(&sheet, 1.0, false)?;
    Ok(sheet)
}
