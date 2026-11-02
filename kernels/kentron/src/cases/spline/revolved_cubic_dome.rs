//! A dome turned a full turn about the z axis from the region in the xz plane (x ≥ 0) bounded by
//! the z axis from (0, 0, 0) to (0, 0, 10), the x axis from (0, 0, 0) to (9, 0, 0), and the cubic
//! Bézier curve with control points (9, 0, 0), (12, 0, 4), (9, 0, 10), (0, 0, 10). It overhangs its
//! round base of radius 9 and closes smoothly over the top at (0, 0, 10).
use crate::bench::p;
use crate::shared::spline::bezier;
use kentron::make;
use kentron::math::Axis;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let side = bezier(&[(9.0, 0.0, 0.0), (12.0, 0.0, 4.0), (9.0, 0.0, 10.0), (0.0, 0.0, 10.0)]);
    let side = make::edge_from_curve(m, side)?;
    let axis = make::edge_between(m, p(0.0, 0.0, 10.0), p(0.0, 0.0, 0.0))?;
    let floor = make::edge_between(m, p(0.0, 0.0, 0.0), p(9.0, 0.0, 0.0))?;
    let outline = make::wire(m, &[side, axis, floor])?;
    let region = make::planar_face(m, outline)?;
    Ok(sweep::revolve(m, region, Axis::Z, TAU)?.shape)
}
