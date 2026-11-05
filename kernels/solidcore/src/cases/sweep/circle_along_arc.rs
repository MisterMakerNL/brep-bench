//! A circle of radius 2 swept a quarter turn along the arc of radius 10 about the z axis from
//! (10, 0, 0) to (0, 10, 0), square to the path, its ends flat.
use crate::bench::{arc, circle};
use crate::Refusal;
use solidcore_ops::sweep::{sweep_placed, SweepFrame};
use solidcore_ops::Body;
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<Body, Refusal> {
    // the circle about (10, 0, 0), square to the path there (along +y)
    let section = circle([10.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 2.0);
    let path = arc([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 10.0, 0.0, FRAC_PI_2);
    let (body, _, _) = sweep_placed(&[section], &[path], SweepFrame::Follow, true)?;
    Ok(body)
}
