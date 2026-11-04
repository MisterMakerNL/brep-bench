//! An L-shaped bracket 10 thick: its outline in the xz plane (a post 20 wide and 120 tall, an arm
//! 20 deep reaching to x = 100 along its top, the inside corner rounded to radius 8 about
//! (28, 92)) drawn at y = −5 and extruded 10 along y.
use crate::bench::{arc, line};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;
use std::f64::consts::PI;

pub fn build() -> Result<Body, Refusal> {
    let p = |x: f64, z: f64| [x, -5.0, z];
    // the round, angles from +x toward −z about +y: from (20, 92) at 180° to (28, 100) at 270°
    let round = arc(p(28.0, 92.0), [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 8.0, PI, 1.5 * PI);
    let outline = vec![
        line(p(0.0, 0.0), p(20.0, 0.0)),
        line(p(20.0, 0.0), p(20.0, 92.0)),
        round,
        line(p(28.0, 100.0), p(100.0, 100.0)),
        line(p(100.0, 100.0), p(100.0, 120.0)),
        line(p(100.0, 120.0), p(0.0, 120.0)),
        line(p(0.0, 120.0), p(0.0, 0.0)),
    ];
    Ok(extrude(&outline, [0.0, 1.0, 0.0], 10.0, 0.0, DraftCorner::Curvature, true)?)
}
