//! A pipe of radius 3 along two straight runs, from (−30, 0, 0) to the origin and on to
//! (0, 30, 0), the section carried around the corner by turning it about the z axis: the two
//! runs, each the disk extruded along it, and the wedge outside the corner filled by the half
//! disk turned a quarter turn about the z axis, a quarter ball. The case composes it so.
use crate::bench::{arc, circle, fuse_all, line};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, revolve, DraftCorner};
use solidcore_ops::Body;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> Result<Body, Refusal> {
    let first = circle([-30.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 3.0);
    let first = extrude(&[first], [1.0, 0.0, 0.0], 30.0, 0.0, DraftCorner::Curvature, true)?;
    let second = circle([0.0; 3], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    let second = extrude(&[second], [0.0, 1.0, 0.0], 30.0, 0.0, DraftCorner::Curvature, true)?;
    // the half of the first run's end disk where y ≤ 0, in the plane x = 0, turned from square
    // to the first run (+x) to square to the second (+y): it sweeps x ≥ 0, y ≤ 0
    let half = [
        // about −x, angle 0 along −z: from (0, 0, −3) through (0, −3, 0) to (0, 0, 3)
        arc([0.0; 3], [-1.0, 0.0, 0.0], [0.0, 0.0, -1.0], 3.0, 0.0, PI),
        line([0.0, 0.0, 3.0], [0.0, 0.0, -3.0]),
    ];
    let joint = revolve(&half, [0.0; 3], [0.0, 0.0, 1.0], FRAC_PI_2, true)?;
    fuse_all([first, joint, second])
}
