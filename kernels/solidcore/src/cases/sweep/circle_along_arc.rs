//! A circle of radius 2 swept along a quarter turn of the arc of radius 10 about the z axis in the
//! xy plane, from (10, 0, 0) to (0, 10, 0), its section square to the path, its ends flat.
use crate::bench::{arc, circle};
use solidcore_ops::sweep::{sweep_placed, SweepFrame};
use solidcore_ops::{Body, Result};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<Body> {
    // the section at the path's start, square to it there (+y)
    let section = circle([10.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 2.0);
    let path = arc([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 10.0, 0.0, FRAC_PI_2);
    let (pipe, _, _) = sweep_placed(&[section], &[path], SweepFrame::Follow, true)?;
    Ok(pipe)
}
