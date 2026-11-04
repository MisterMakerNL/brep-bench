//! A hollow square duct bent through a quarter turn: the square 18 × 18 with a square hole
//! 14 × 14, standing in the xz plane centred on (30, 0, 0), swept along the arc of radius 30
//! about the z axis from (30, 0, 0) to (0, 30, 0), its section in the plane through the axis.
use crate::bench::{arc, polygon};
use solidcore_ops::sweep::{sweep_placed, SweepFrame};
use solidcore_ops::{Body, Result};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<Body> {
    // the section: the outside square and the hole, each given in the xz plane as (x, z)
    let square = |h: f64| {
        polygon(&[[30.0 - h, 0.0, -h], [30.0 + h, 0.0, -h], [30.0 + h, 0.0, h], [30.0 - h, 0.0, h]])
    };
    let section = [square(9.0), square(7.0)].concat();
    let path = arc([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 30.0, 0.0, FRAC_PI_2);
    let (duct, _, _) = sweep_placed(&section, &[path], SweepFrame::Follow, true)?;
    Ok(duct)
}
