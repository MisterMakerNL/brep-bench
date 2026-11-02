//! An L-shaped plate, the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
//! extruded 8 along +z, shelled inward to a wall 1 thick, open at its top face, z = 8: the wall is
//! every point of the plate within 1 of its other faces, so at the concave edge x = 5, y = 5 its
//! inside is a quarter cylinder of radius 1 about the edge.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

const OUTLINE: [(f64, f64); 6] = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = shell::plate(m, &OUTLINE, 8.0)?;
    // joined by arcs: the wall round the concave edge is a quarter cylinder about it
    shell::inward(m, plate, &[p(2.0, 2.0, 8.0)], 1.0, Join::Arc)
}
