//! A hollow square duct bent through a quarter turn: the square 18 × 18 with a square hole
//! 14 × 14 at its center, standing in the xz plane centered on (30, 0, 0), swept along the arc
//! of radius 30 about the z axis from (30, 0, 0) to (0, 30, 0), turning with it.
use crate::bench::{arc, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{sweep_placed, SweepFrame};
use solidcore_ops::Body;
use std::f64::consts::FRAC_PI_2;

/// The square of half-width `h` in the xz plane centered on (30, 0, 0).
fn square(h: f64) -> Vec<solidcore_ops::sweep::Curve> {
    let (a, b) = (30.0 - h, 30.0 + h);
    polygon(&[[a, 0.0, -h], [b, 0.0, -h], [b, 0.0, h], [a, 0.0, h]])
}

pub fn build() -> Result<Body, Refusal> {
    let mut section = square(9.0);
    section.extend(square(7.0));
    let path = arc([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 30.0, 0.0, FRAC_PI_2);
    let (body, _, _) = sweep_placed(&section, &[path], SweepFrame::Follow, true)?;
    Ok(body)
}
