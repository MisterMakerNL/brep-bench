//! The inner half of the torus about the z axis whose tube has radius 2 about a middle circle of
//! radius 6 (the points of it no farther than 6 from the axis), a sheet of one face, thickened
//! by 1 away from the middle circle, toward the axis.
use crate::bench::arc;
use crate::shared::thicken::thickened;
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<Body, Refusal> {
    // the half circle about (6, 0, 0) from (6, 0, 2) through (4, 0, 0) to (6, 0, −2): about −y,
    // angle 0 along +z
    let meridian = arc([6.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], 2.0, 0.0, PI);
    let sheet = revolve(&[meridian], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    // the turned face's normal points away from the middle circle
    thickened(sheet, 1.0, false)
}
