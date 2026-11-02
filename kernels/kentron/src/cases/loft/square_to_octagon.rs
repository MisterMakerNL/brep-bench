//! A square-to-octagon transition: the square −20 ≤ x, y ≤ 20 at z = 0 joined to the octagon at
//! z = 30 (the square 28 × 28 with its corners cut off, corners (6, −14), (14, −6), ...), each
//! side of the square to the octagon's side parallel to it by a flat trapezoid, each corner of
//! the square to the cut side facing it by a flat triangle: the convex hull of the two.
//!
//! kentron's loft joins its sections edge to edge (splitting edges to match), so it cannot pair
//! a corner of the square with a whole side of the octagon: the hull is sewn from its ten flat
//! faces.
use crate::bench::p;
use crate::shared::geometry::solid_of;
use crate::shared::loft::flat;
use crate::Refusal;
use kentron::math::Point;
use kentron::topology::{Model, Shape};

const HEIGHT: f64 = 30.0;
/// Counter-clockwise from the corner at (20, −20); each corner of the square is followed by
/// the octagon's side facing it, then the next side of the octagon, parallel to the square's
/// next side.
const SQUARE: [(f64, f64); 4] = [(20.0, -20.0), (20.0, 20.0), (-20.0, 20.0), (-20.0, -20.0)];
const OCTAGON: [(f64, f64); 8] =
    [(6.0, -14.0), (14.0, -6.0), (14.0, 6.0), (6.0, 14.0), (-6.0, 14.0), (-14.0, 6.0), (-14.0, -6.0), (-6.0, -14.0)];

fn at((x, y): (f64, f64), z: f64) -> Point {
    p(x, y, z)
}

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let floor: Vec<Point> = SQUARE.iter().rev().map(|&c| at(c, 0.0)).collect();
    let top: Vec<Point> = OCTAGON.iter().map(|&c| at(c, HEIGHT)).collect();
    let mut faces = vec![flat(m, &floor)?, flat(m, &top)?];
    for (k, &corner) in SQUARE.iter().enumerate() {
        let before = OCTAGON[(2 * k + 7) % 8]; // the octagon's side parallel to the square's,
        let after = OCTAGON[2 * k]; //            from `before` to `after`
        let side_end = OCTAGON[2 * k + 1]; // the cut side facing the corner, from `after`
        let prev = SQUARE[(k + 3) % 4];
        faces.push(flat(m, &[at(prev, 0.0), at(corner, 0.0), at(after, HEIGHT), at(before, HEIGHT)])?);
        faces.push(flat(m, &[at(corner, 0.0), at(side_end, HEIGHT), at(after, HEIGHT)])?);
    }
    solid_of(m, &faces)
}
