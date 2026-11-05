//! The side of the cone frustum about the z axis whose radius runs from 6 at z = 0 to 3 at
//! z = 4, a sheet of one face, thickened by 1 along its normal away from the axis.
use crate::bench::line;
use crate::shared::thicken::thickened;
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;
use std::f64::consts::TAU;

pub fn build() -> Result<Body, Refusal> {
    let slant = line([6.0, 0.0, 0.0], [3.0, 0.0, 4.0]);
    let sheet = revolve(&[slant], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    thickened(sheet, 1.0, false)
}
