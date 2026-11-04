//! A quarter of a washer: the rectangle with x from 6 to 10 and z from 0 to 4 in the xz plane,
//! revolved 90° about the z axis, right-handed (from +x towards +y).
use crate::bench::polygon;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{Body, Result};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<Body> {
    let section = polygon(&[[6.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 4.0], [6.0, 0.0, 4.0]]);
    revolve(&section, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], FRAC_PI_2, true)
}
