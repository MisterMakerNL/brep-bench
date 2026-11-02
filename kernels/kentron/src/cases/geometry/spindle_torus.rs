//! A solid spindle torus about the z axis, middle circle radius 3 and tube radius 5, counted
//! once: what the disc (x − 3)² + z² ≤ 25 fills as it turns, which is the turn of its part with
//! x ≥ 0, the outer arc from (0, 0, −4) through (8, 0, 0) to (0, 0, 4) closed by the axis.
use crate::bench::p;
use crate::shared::geometry::{flat, segment, turned};
use crate::Refusal;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (south, north) = (p(0.0, 0.0, -4.0), p(0.0, 0.0, 4.0));
    let arc = Circle::arc_through(south, p(8.0, 0.0, 0.0), north).map_err(|e| e.to_string())?;
    let arc = make::edge_from_curve(m, Curve::Trimmed(arc))?;
    let axis = segment(m, north, south)?;
    let section = flat(m, &[arc, axis])?;
    Ok(turned(m, section)?)
}
