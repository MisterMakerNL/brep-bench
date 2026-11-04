//! A washer: the rectangle with x from 6 to 10 and z from 0 to 4 in the xz plane, revolved a full
//! turn about the z axis.
use crate::bench::polygon;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{Body, Result};
use std::f64::consts::TAU;

pub fn build() -> Result<Body> {
    let section = polygon(&[[6.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 4.0], [6.0, 0.0, 4.0]]);
    revolve(&section, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], TAU, true)
}
