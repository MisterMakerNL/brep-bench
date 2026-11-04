//! The bent sheet of `thicken/bent-flange` (a floor, a quarter cylinder of radius 2 about the line
//! through (0, ·, 2) along y, and an upright wall) thickened by 3 to the inside of the bend, past
//! its radius: the floor and wall moved 3 meet at a concave edge, the bend's offset left out.
use crate::shared::thicken::flange;
use solidcore_ops::{thicken, Body, Result};

pub fn build() -> Result<Body> {
    let sheet = flange()?;
    // its normals face toward the bend's axis
    thicken(&sheet, 3.0, false)?;
    Ok(sheet)
}
