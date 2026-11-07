//! A square column from z = 0 to z = 40 whose section at height z is the square of side
//! 30 − z/2 centered on the z axis, turned 4.5° per unit of height (right-handed about +z): from
//! the square −15 ≤ x, y ≤ 15 at its foot to −5 ≤ x, y ≤ 5 at its top, half a turn round.
//!
//! The square at its foot swept up the z axis with cadrs's `Kernel::sweep_with_law`, twisting
//! half a turn about the path and scaled to a third, both in proportion to the height: its
//! corners run along the four conical spirals, its sides sweep the twisted ruled walls.
use crate::bench::{line, p2, rectangle, sketch, xy, xz};
use crate::shared::sweep::{on, sweep_law};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
use std::f64::consts::PI;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let foot = sketch(xy(0.0), vec![rectangle(-15.0, -15.0, 15.0, 15.0)]);
    // The z axis from 0 to 40, drawn on the Front plane (its sketch y is the model's z).
    let path = vec![on(xz(0.0), line(p2(0.0, 0.0), p2(0.0, 40.0)))];
    sweep_law(k, &foot, path, PI, 1.0 / 3.0)
}
