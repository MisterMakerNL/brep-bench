//! The cap of the sphere of radius 10 about the origin where z ≥ 6, a sheet of one face bounded
//! by the circle of radius 8 at z = 6, thickened by 2 away from the center.
use crate::bench::arc;
use crate::shared::thicken::thickened;
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<Body, Refusal> {
    // the meridian from (8, 0, 6) up to (0, 0, 10): about −y, angle 0 along +x
    let start = 0.6f64.asin();
    let meridian = arc([0.0; 3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 10.0, start, FRAC_PI_2);
    let sheet = revolve(&[meridian], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    // the turned face's normal points away from the center
    thickened(sheet, 2.0, false)
}
