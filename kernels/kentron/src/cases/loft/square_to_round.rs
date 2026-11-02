//! A square-to-round transition: the square −20 ≤ x, y ≤ 20 at z = 0 joined to the circle of
//! radius 15 about the z axis at z = 30, each side of the square by a flat triangle to the
//! point of the circle nearest it, each corner by a piece of an oblique cone (its apex the
//! corner) to the quarter of the circle between the points beside it: the convex hull of the
//! square and the circle.
//!
//! kentron's loft joins its sections edge to edge, so one loft of the square to the circle
//! cannot pair each side with a point and each corner with an arc: each cone is a ruled loft
//! from its corner (a vertex section) to its quarter circle, each triangle is flat, and the
//! hull is sewn from them, the floor and the disc.
use crate::bench::p;
use crate::shared::geometry::solid_of;
use crate::shared::loft::flat;
use crate::Refusal;
use kentron::geometry::curve::{Circle, Curve};
use kentron::loft::{self, LoftOptions};
use kentron::make;
use kentron::math::{Direction, Frame, Point};
use kentron::topology::{Model, NodeKind, Shape};
use std::f64::consts::{FRAC_PI_2, TAU};

const HALF: f64 = 20.0; // half the square's side
const RADIUS: f64 = 15.0;
const HEIGHT: f64 = 30.0;

fn rim() -> Circle {
    Circle::new(Frame::new(p(0.0, 0.0, HEIGHT), Direction::Z, Direction::X), RADIUS)
}

/// The piece of the cone from `apex` to the quarter of the circle starting at the angle
/// `start` and turning a quarter turn counter-clockwise: a ruled loft from the point to the
/// arc, each point of the arc joined to the apex by a straight line.
fn cone(m: &mut Model, apex: Point, start: f64) -> Result<Shape, Refusal> {
    let arc = make::edge_from_curve_range(m, Curve::Circle(rim()), start, start + FRAC_PI_2)?;
    let arc = make::wire(m, &[arc])?;
    let apex = make::vertex(m, apex);
    let options = LoftOptions { ruled: true, ..LoftOptions::default() };
    let shell = loft::loft(m, &[apex, arc], &options)?.shape;
    match &m.unique_subshapes(shell, NodeKind::Face)[..] {
        [face] => Ok(*face),
        faces => Err(format!("the loft of a corner to its quarter circle made {} faces", faces.len()).into()),
    }
}

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let h = HALF;
    let corners = [p(h, -h, 0.0), p(h, h, 0.0), p(-h, h, 0.0), p(-h, -h, 0.0)]; // counter-clockwise
    let floor: Vec<Point> = corners.iter().rev().copied().collect();
    let mut faces = vec![flat(m, &floor)?];
    let top = make::edge_from_curve(m, Curve::Circle(rim()))?;
    let top = make::wire(m, &[top])?;
    faces.push(make::planar_face(m, top)?);
    for (k, &corner) in corners.iter().enumerate() {
        // the corner at angle −45° + 90°k sees the quarter circle from −90° + 90°k
        let start = (-FRAC_PI_2 + k as f64 * FRAC_PI_2).rem_euclid(TAU);
        faces.push(cone(m, corner, start)?);
        // the triangle from the side before this corner to the circle's point nearest that side
        let near = p(RADIUS * start.cos(), RADIUS * start.sin(), HEIGHT);
        faces.push(flat(m, &[corners[(k + 3) % 4], corner, near])?);
    }
    solid_of(m, &faces)
}
