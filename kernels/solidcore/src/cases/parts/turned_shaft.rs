//! A stepped shaft on the z axis: its section in the xz plane (radius 20 from z = 0 to 60, radius
//! 12 from 60 to 160, the inside corner of the shoulder rounded to radius 4 about (16, 64))
//! turned a whole turn about the z axis.
use crate::bench::{arc, line};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<Body, Refusal> {
    let p = |x: f64, z: f64| [x, 0.0, z];
    // the round, angles from +x toward +z about −y: from (12, 64) at −180° to (16, 60) at −90°
    let round = arc(p(16.0, 64.0), [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 4.0, -PI, -PI / 2.0);
    let section = vec![
        line(p(0.0, 0.0), p(20.0, 0.0)),
        line(p(20.0, 0.0), p(20.0, 60.0)),
        line(p(20.0, 60.0), p(16.0, 60.0)),
        round,
        line(p(12.0, 64.0), p(12.0, 160.0)),
        line(p(12.0, 160.0), p(0.0, 160.0)),
        line(p(0.0, 160.0), p(0.0, 0.0)),
    ];
    Ok(revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], TAU, true)?)
}
