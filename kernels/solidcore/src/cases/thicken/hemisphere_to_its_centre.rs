//! The upper half of the sphere of radius 5 about the origin, one face bounded by its equator,
//! thickened by 5 toward the centre, its whole radius: the half ball of radius 5 where z >= 0.
use crate::bench::arc;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{thicken, Body, Result};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<Body> {
    // the quarter circle in the xz plane from (5, 0, 0) to the pole, turned about the z axis
    // uncapped
    let meridian = arc([0.0; 3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 5.0, 0.0, PI / 2.0);
    let dome = revolve(&[meridian], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    // its normal faces away from the centre: toward it is against the normal
    thicken(&dome, -5.0, false)?;
    Ok(dome)
}
