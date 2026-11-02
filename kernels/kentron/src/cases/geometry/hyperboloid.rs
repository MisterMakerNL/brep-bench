//! The solid inside the hyperboloid of one sheet x² + y² ≤ 16(1 + z²/36), between z = −6 and
//! z = 6: the hyperbola x²/16 − z²/36 = 1 of the xz plane (its branch with x > 0), closed by the
//! ends' radii and the axis, turned a whole turn about the z axis.
use crate::bench::p;
use crate::shared::geometry::{arc, flat, segment, turned};
use kentron::geometry::curve::{Curve, Hyperbola};
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};

const A: f64 = 4.0; // the waist's radius
const C: f64 = 6.0; // the other semi-axis
const H: f64 = 6.0; // the ends are at z = ±H

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // (A cosh t, C sinh t) along the frame's x and y: +x, and +z when its normal is −y
    let frame = Frame::new(p(0.0, 0.0, 0.0), -Direction::Y, Direction::X);
    let branch = Curve::Hyperbola(Hyperbola::new(frame, A, C));
    let end = (H / C).asinh();
    let side = arc(m, branch, -end, end)?;
    let rim = A * (1.0 + (H / C).powi(2)).sqrt();
    let top = segment(m, p(rim, 0.0, H), p(0.0, 0.0, H))?;
    let axis = segment(m, p(0.0, 0.0, H), p(0.0, 0.0, -H))?;
    let floor = segment(m, p(0.0, 0.0, -H), p(rim, 0.0, -H))?;
    let section = flat(m, &[side, top, axis, floor])?;
    turned(m, section)
}
