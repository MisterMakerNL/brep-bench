//! A block, the box from (−6, −6, −8) to (18, 18, 0), grooved by a ball of radius 3 whose
//! centre swings a quarter turn about the z axis in the top face, from (12, 0, 0) to (0, 12, 0):
//! everything the ball passes through cut from the block.
//!
//! solidcore has no sweep of a solid, so the ball's sweep is built as what it is: the tube about
//! its centre's path, the disk of radius 3 square to the arc at (12, 0, 0) turned the quarter
//! turn about the z axis, and beyond each of its ends the half of the ball there that the tube
//! leaves out. (Whole balls, whose great circles lie on the tube's end disks, the kernel's
//! booleans do not cut: status 1001.)
use crate::bench::{arc, circle, cut_all, line};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{make_box, Body};
use std::f64::consts::{FRAC_PI_2, TAU};

/// The half of the ball of radius 3 about `c` on the side `out` of it (a unit vector square
/// to z), turned whole about the line through `c` along `out`: the quarter disk in the plane
/// z = 0 between that line and the direction `side`, square to it.
fn half_ball(c: [f64; 3], out: [f64; 3], side: [f64; 3]) -> Result<Body, Refusal> {
    let at = |a: f64, b: f64| [c[0] + a * out[0] + b * side[0], c[1] + a * out[1] + b * side[1], c[2]];
    // counter-clockwise about out × side from out to side
    let normal = [out[1] * side[2] - out[2] * side[1], out[2] * side[0] - out[0] * side[2], out[0] * side[1] - out[1] * side[0]];
    let quarter = [line(c, at(3.0, 0.0)), arc(c, normal, out, 3.0, 0.0, FRAC_PI_2), line(at(0.0, 3.0), c)];
    Ok(revolve(&quarter, c, out, TAU, true)?)
}

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-6.0, -6.0, -8.0], [24.0, 24.0, 8.0])?;
    let disk = circle([12.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 3.0);
    let tube = revolve(&[disk], [0.0; 3], [0.0, 0.0, 1.0], FRAC_PI_2, true)?;
    // the tube starts square to +y at (12, 0, 0) and ends square to −x at (0, 12, 0)
    let start = half_ball([12.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0])?;
    let end = half_ball([0.0, 12.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    cut_all(block, [tube, start, end])
}
