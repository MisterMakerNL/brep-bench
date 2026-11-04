//! The inner half of the torus about the z axis whose tube has radius 2 about a middle circle of
//! radius 6, the points of it no farther than 6 from the axis, one saddle face, thickened by 1
//! away from the middle circle, toward the axis.
use crate::bench::arc;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{thicken, Body, Result};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<Body> {
    // the half circle in the xz plane about (6, 0, 0) from (6, 0, 2) through (4, 0, 0) to
    // (6, 0, -2), turned a full turn about the z axis uncapped
    let section =
        arc([6.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 2.0, PI / 2.0, 3.0 * PI / 2.0);
    let sheet = revolve(&[section], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    // its normal faces away from the middle circle
    thicken(&sheet, 1.0, false)?;
    Ok(sheet)
}
