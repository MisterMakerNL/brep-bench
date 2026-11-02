//! A square spire lofted from a square at its foot to a point at its top on four ridges, z from
//! 0 to 24: each section a square centred on the z axis with sides parallel to x and y, its
//! corners on the cubic Bézier ridge (9, 9, 0), (15, 15, 8), (12, 12, 19), (0, 0, 24) and its
//! mirror images in the xz plane, the yz plane and both. It swells from 18 × 18 to 24 × 24 and
//! closes to the apex (0, 0, 24).
//!
//! kentron's loft passes through sections but follows no guides, so the solid is built from
//! what its walls are: the section's sides stay parallel to x and y, so the walls facing ±x are
//! the ridge seen along y, (g, z), and its mirror image, each slid along y, and those facing ±y
//! the ridge seen along x, slid along x. The solid is the common of the two extrusions.
use crate::bench::{p, v};
use crate::shared::loft::between;
use kentron::boolean;
use kentron::topology::{Model, Shape};

const REACH: f64 = 40.0; // how far each extrusion runs past the solid, either way
/// The ridge through the corner with x and y positive, (g, g, z): its control points.
const RIDGE: [(f64, f64, f64); 4] = [(9.0, 9.0, 0.0), (15.0, 15.0, 8.0), (12.0, 12.0, 19.0), (0.0, 0.0, 24.0)];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let seen_along_y: Vec<_> = RIDGE.iter().map(|&(x, _, z)| p(x, 0.0, z)).collect();
    let seen_along_x: Vec<_> = RIDGE.iter().map(|&(_, y, z)| p(0.0, y, z)).collect();
    let walls_x = between(m, &seen_along_y, |q| p(-q.x, q.y, q.z), v(0.0, REACH, 0.0))?;
    let walls_y = between(m, &seen_along_x, |q| p(q.x, -q.y, q.z), v(REACH, 0.0, 0.0))?;
    boolean::intersection(m, walls_x, walls_y)
}
