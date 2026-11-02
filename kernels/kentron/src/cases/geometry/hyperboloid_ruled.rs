//! The solid bounded by two flat discs of radius 5 about the z axis, at z = −5 and z = 5, and the
//! surface ruled between their rims, each point at angle θ below joined by a straight line to
//! the point at θ + 90° above: a hyperboloid of one sheet, built as the ruled surface. The rims
//! are circles whose parameter 0 lies at angles 0 and 90°, and a ruled loft between them, the
//! sections taken as given, joins the points at the same parameter.
use crate::bench::{d, p};
use kentron::geometry::curve::{Circle, Curve};
use kentron::loft::{self, LoftOptions, Matching};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};

const RADIUS: f64 = 5.0;
const H: f64 = 5.0; // the rims are at z = ±H
const TWIST: f64 = 90.0; // degrees: the upper end of each line is this far round from its lower end

/// The circle of radius RADIUS about z at height `z`, its parameter 0 at angle `start`.
fn rim(m: &mut Model, z: f64, start: f64) -> kentron::Result<Shape> {
    let a = start.to_radians();
    let frame = Frame::new(p(0.0, 0.0, z), Direction::Z, d(a.cos(), a.sin(), 0.0));
    let edge = make::edge_from_curve(m, Curve::Circle(Circle::new(frame, RADIUS)))?;
    make::wire(m, &[edge])
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let lower = rim(m, -H, 0.0)?;
    let upper = rim(m, H, TWIST)?;
    let options = LoftOptions { solid: true, ruled: true, matching: Matching::AsGiven, ..LoftOptions::default() };
    Ok(loft::loft(m, &[lower, upper], &options)?.shape)
}
