//! A solid lofted through rectangles on four guides, z from 0 to 24: each section a rectangle
//! centred on the z axis with sides parallel to x and y, its corners on the cubic Bézier guide
//! (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19), (3, 12.2, 24) and its mirror images in the xz
//! plane, the yz plane and both. It begins 24 × 6.4, pinches to 30 × 0.4 and ends 6 × 24.4.
//!
//! kentron's loft passes through sections but follows no guides, so the solid is built from
//! what its walls are: the section's sides stay parallel to x and y, so the walls facing ±x are
//! the guide seen along y, (p, z), and its mirror image, each slid along y, and those facing ±y
//! are the guide seen along x, (q, z), and its mirror image, slid along x. The solid is
//! everything between both pairs: the common of the two extrusions.
use crate::bench::{p, v};
use crate::shared::loft::between;
use kentron::boolean;
use kentron::topology::{Model, Shape};

const REACH: f64 = 40.0; // how far each extrusion runs past the solid, either way
/// The guide through the corner with x and y positive, (p, q, z): its control points.
const GUIDE: [(f64, f64, f64); 4] = [(12.0, 3.2, 0.0), (18.0, -2.8, 8.0), (15.0, 0.2, 19.0), (3.0, 12.2, 24.0)];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let seen_along_y: Vec<_> = GUIDE.iter().map(|&(x, _, z)| p(x, 0.0, z)).collect();
    let seen_along_x: Vec<_> = GUIDE.iter().map(|&(_, y, z)| p(0.0, y, z)).collect();
    let walls_x = between(m, &seen_along_y, |q| p(-q.x, q.y, q.z), v(0.0, REACH, 0.0))?;
    let walls_y = between(m, &seen_along_x, |q| p(q.x, -q.y, q.z), v(REACH, 0.0, 0.0))?;
    boolean::intersection(m, walls_x, walls_y)
}
