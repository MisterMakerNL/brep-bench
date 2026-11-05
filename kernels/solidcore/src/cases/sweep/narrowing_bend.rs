//! A hollow square duct narrowing as it bends through a quarter turn about the z axis, its
//! centerline the arc of radius 30 from (30, 0, 0) to (0, 30, 0). At an angle θ turned, the
//! section stands in the vertical plane through the z axis at that angle: the square of
//! half-width h = 9 − 3·θ/90° centered on the arc, less the square of half-width h − 2.
//!
//! solidcore's sweep of a section that changes as it goes is `sweep_stations`: the section
//! placed by an affine map at each station, the sections skinned through. Each square is the
//! square at the start (in the xz plane, about (30, 0, 0)) scaled about its center and turned
//! about the z axis: one map for the outside, scaled by h/9, and another for the hole, scaled by
//! (h − 2)/7, so the outside and the hole are swept apart and the hole cut from the outside.
use crate::bench::{cut, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{sweep_stations, Affine, Curve, Ends};
use solidcore_ops::Body;
use std::f64::consts::FRAC_PI_2;

/// The stations: one at every degree turned.
const STATIONS: usize = 91;

/// The square of half-width `h` in the xz plane centered on (30, 0, 0).
fn square(h: f64) -> Vec<Curve> {
    let (a, b) = (30.0 - h, 30.0 + h);
    polygon(&[[a, 0.0, -h], [b, 0.0, -h], [b, 0.0, h], [a, 0.0, h]])
}

/// The square of half-width `start` at θ = 0, swept to half-width `end` at θ = 90°, its
/// half-width running linearly with θ: at each station it is scaled by its half-width there
/// over `start` about (30, 0, 0), then turned by θ about the z axis.
fn narrowing(start: f64, end: f64) -> Result<Body, Refusal> {
    let stations: Vec<Affine> = (0..STATIONS)
        .map(|i| {
            let t = i as f64 / (STATIONS - 1) as f64;
            let (s, c) = (t * FRAC_PI_2).sin_cos();
            let k = (start + (end - start) * t) / start;
            Affine {
                m: [[k * c, k * s, 0.0], [-k * s, k * c, 0.0], [0.0, 0.0, k]],
                t: [(1.0 - k) * 30.0 * c, (1.0 - k) * 30.0 * s, 0.0],
            }
        })
        .collect();
    Ok(sweep_stations(&square(start), &stations, Ends::Capped)?)
}

pub fn build() -> Result<Body, Refusal> {
    cut(narrowing(9.0, 6.0)?, narrowing(7.0, 4.0)?)
}
