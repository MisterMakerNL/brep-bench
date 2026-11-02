//! A solid oblate spheroid about the z axis, (x² + y²)/36 + z²/9 ≤ 1: the half of the ellipse
//! x²/36 + z²/9 = 1 of the xz plane with x ≥ 0, closed by the axis, turned a whole turn about z.
use crate::bench::p;
use crate::shared::geometry::{arc, flat, segment, turned};
use kentron::geometry::curve::{Curve, Ellipse};
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

const EQUATOR: f64 = 6.0;
const POLE: f64 = 3.0;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // (EQUATOR cos t, POLE sin t) along the frame's x and y: +x, and +z when its normal is −y
    let frame = Frame::new(p(0.0, 0.0, 0.0), -Direction::Y, Direction::X);
    let ellipse = Curve::Ellipse(Ellipse::new(frame, EQUATOR, POLE));
    let half = arc(m, ellipse, -FRAC_PI_2, FRAC_PI_2)?;
    let axis = segment(m, p(0.0, 0.0, POLE), p(0.0, 0.0, -POLE))?;
    let section = flat(m, &[half, axis])?;
    turned(m, section)
}
