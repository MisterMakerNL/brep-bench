//! A hollow square duct bent a quarter turn round the arc of radius 30 about the z axis, from
//! (30, 0, 0) to (0, 30, 0): the section, 18 × 18 with a 14 × 14 hole, standing in the xz plane
//! at x = 30, turned a quarter turn about the z axis from +x towards +y.
use crate::bench::p;
use crate::shared::sweep::holed_polygon;
use kentron::math::{Axis, Direction, Point};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

/// The square of half-side `h` in the xz plane, centred at (30, 0, 0).
fn square(h: f64) -> Vec<Point> {
    vec![p(30.0 - h, 0.0, -h), p(30.0 + h, 0.0, -h), p(30.0 + h, 0.0, h), p(30.0 - h, 0.0, h)]
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let section = holed_polygon(m, &square(9.0), &square(7.0))?;
    Ok(sweep::revolve(m, section, Axis::new(p(0.0, 0.0, 0.0), Direction::Z), FRAC_PI_2)?.shape)
}
