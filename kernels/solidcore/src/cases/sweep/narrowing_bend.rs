//! A hollow square duct narrowing as it bends through a quarter turn about the z axis along the
//! arc of radius 30 from (30, 0, 0) to (0, 30, 0): its section, in the vertical plane through the
//! axis, a square of outer half-width h = 9 - 3·θ/90° centred on the arc, with a square hole of
//! half-width h - 2; its ends flat.
use crate::bench::{cut, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{sweep_stations, Affine, Ends};
use solidcore_ops::Body;
use std::f64::consts::FRAC_PI_2;

/// Stations along the quarter turn: one every half degree.
const STATIONS: usize = 181;

/// The square of half-width `h` in the xz plane centred on (30, 0, 0), swept round the quarter
/// turn by stations each turning it by θ about the z axis and scaling it about the arc's point,
/// so that its half-width there is `half_width(θ)`.
fn swept_square(h: f64, half_width: impl Fn(f64) -> f64) -> Result<Body, Refusal> {
    let square = polygon(&[
        [30.0 - h, 0.0, -h],
        [30.0 + h, 0.0, -h],
        [30.0 + h, 0.0, h],
        [30.0 - h, 0.0, h],
    ]);
    let stations: Vec<Affine> = (0..STATIONS)
        .map(|i| {
            let theta = FRAC_PI_2 * i as f64 / (STATIONS - 1) as f64;
            let (sin, cos) = theta.sin_cos();
            let k = half_width(theta) / h;
            // p -> R(θ)·(c + k·(p - c)), c = (30, 0, 0): the axes' images k·R(θ)e, the
            // translation (1 - k)·R(θ)c
            Affine {
                m: [[k * cos, k * sin, 0.0], [-k * sin, k * cos, 0.0], [0.0, 0.0, k]],
                t: [(1.0 - k) * 30.0 * cos, (1.0 - k) * 30.0 * sin, 0.0],
            }
        })
        .collect();
    Ok(sweep_stations(&square, &stations, Ends::Capped)?)
}

pub fn build() -> Result<Body, Refusal> {
    // ops has no sweep with a law for its section's size; the section placed at stations along
    // the turn, each turned and scaled as the law says there, and skinned through, is the
    // nearest it has
    let outer = |theta: f64| 9.0 - 3.0 * theta / FRAC_PI_2;
    let outside = swept_square(9.0, outer)?;
    let hole = swept_square(7.0, |theta| outer(theta) - 2.0)?;
    cut(outside, hole)
}
