//! A lemon: the circular segment of the disc of radius 5 about (−3, 0, 0) in the xz plane with
//! x ≥ 0 (its arc from (0, 0, −4) through (2, 0, 0) to (0, 0, 4), its chord on the z axis),
//! turned a whole turn about the z axis.
use crate::bench::p;
use crate::shared::geometry::{flat, segment, turned};
use crate::Refusal;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (south, north) = (p(0.0, 0.0, -4.0), p(0.0, 0.0, 4.0));
    let arc = Circle::arc_through(south, p(2.0, 0.0, 0.0), north).map_err(|e| e.to_string())?;
    let arc = make::edge_from_curve(m, Curve::Trimmed(arc))?;
    let chord = segment(m, north, south)?;
    let section = flat(m, &[arc, chord])?;
    Ok(turned(m, section)?)
}
