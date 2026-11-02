//! A circle of radius 2 swept along a quarter turn of the arc of radius 10 about the z axis in
//! the xy plane, from (10, 0, 0) to (0, 10, 0), the section square to the path and the ends flat:
//! the disc square to the arc where it starts, piped along it.
use crate::bench::p;
use crate::shared::sweep::disc;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // The arc: the circle about the z axis, angle 0 on +x, from 0 to a quarter turn.
    let circle = Circle::new(Frame::new(p(0.0, 0.0, 0.0), Direction::Z, Direction::X), 10.0);
    let arc = make::edge_from_curve_range(m, Curve::Circle(circle), 0.0, FRAC_PI_2)?;
    // The section where the arc starts, square to it: the arc leaves (10, 0, 0) along +y.
    let section = disc(m, p(10.0, 0.0, 0.0), Direction::Y, 2.0)?;
    Ok(sweep::pipe(m, section, arc)?.shape)
}
