//! A hollow square duct narrowing round a quarter turn of the arc of radius 30 about the z axis,
//! from (30, 0, 0) to (0, 30, 0): each section in the vertical plane through the z axis, a square
//! of half-width h running linearly in the angle from 9 to 6, with a hole of half-width h − 2.
//! Each square swept through its two end sections along the arc (blended linearly along it, so
//! in the angle), the hole's sweep cut from the outside's.
use crate::bench::p;
use crate::shared::sweep::polyline;
use kentron::boolean;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::sweep::{self, PipeOptions};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

/// The solid swept by the square of half-width `start` where the arc starts, in the xz plane
/// about (30, 0, 0), through the square of half-width `end` where it ends, in the yz plane about
/// (0, 30, 0).
fn tapered(m: &mut Model, arc: Shape, start: f64, end: f64) -> kentron::Result<Shape> {
    let (a, b) = (start, end);
    let first = polyline(m, &[p(30.0 - a, 0.0, -a), p(30.0 + a, 0.0, -a), p(30.0 + a, 0.0, a), p(30.0 - a, 0.0, a)], true)?;
    let last = polyline(m, &[p(0.0, 30.0 - b, -b), p(0.0, 30.0 + b, -b), p(0.0, 30.0 + b, b), p(0.0, 30.0 - b, b)], true)?;
    let sections = [(first, PipeOptions::default()), (last, PipeOptions::default())];
    sweep::pipe_through(m, &sections, arc, true)
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let circle = Circle::new(Frame::new(p(0.0, 0.0, 0.0), Direction::Z, Direction::X), 30.0);
    let arc = make::edge_from_curve_range(m, Curve::Circle(circle), 0.0, FRAC_PI_2)?;
    let arc = make::wire(m, &[arc])?;
    let outside = tapered(m, arc, 9.0, 6.0)?;
    // The hole, from end to end: its ends lie in the duct's.
    let hole = tapered(m, arc, 7.0, 4.0)?;
    boolean::difference(m, outside, hole)
}
