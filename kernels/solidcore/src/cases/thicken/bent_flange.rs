//! The bent sheet: the profile in the xz plane along the line from (-10, 0, 0) to the origin,
//! round the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2), and up to (2, 0, 12),
//! swept along +y from y = 0 to 8, thickened by 1 to the outside of the bend: a bracket 1 thick.
use crate::shared::thicken::flange;
use solidcore_ops::{thicken, Body, Result};

pub fn build() -> Result<Body> {
    let sheet = flange()?;
    // its normals face toward the bend's axis: away from it is against them
    thicken(&sheet, -1.0, false)?;
    Ok(sheet)
}
