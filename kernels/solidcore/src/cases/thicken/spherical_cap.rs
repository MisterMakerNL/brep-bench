//! The cap of the sphere of radius 10 about the origin where z >= 6, one face bounded by the
//! circle of radius 8 at z = 6, thickened by 2 away from the centre: the solid between the
//! spheres of radius 10 and 12 inside the cone from the origin through that circle.
use crate::bench::arc;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{thicken, Body, Result};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<Body> {
    // the arc in the xz plane from (8, 0, 6) to the pole (0, 0, 10), turned about the z axis
    // uncapped
    let rim = 6.0f64.atan2(8.0);
    let meridian = arc([0.0; 3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 10.0, rim, PI / 2.0);
    let cap = revolve(&[meridian], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    // its normal faces away from the centre
    thicken(&cap, 2.0, false)?;
    Ok(cap)
}
