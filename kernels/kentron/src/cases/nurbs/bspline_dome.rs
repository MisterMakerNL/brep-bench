//! A block on the square 0 ≤ x, y ≤ 20, its floor flat at z = 0, its sides upright, and its top a
//! clamped, non-rational bicubic B-spline surface of 6 × 5 control points: knots
//! (0, 0, 0, 0, 2/5, 2/5, 1, 1, 1, 1) along u, with a double knot, and (0, 0, 0, 0, 1/3, 1, 1, 1, 1)
//! along v; the control points at x = 0, 3, 7, 11, 16, 20 (rows) and y = 0, 5, 10, 15, 20
//! (columns) at the heights below. The top face is swept straight down past the floor and cut off
//! at the floor.
use crate::bench::p;
use crate::shared::nurbs::block_under;
use kentron::geometry::surface::{BSplineSurface, KnotVector, Surface};
use kentron::topology::{Model, Shape};

const XS: [f64; 6] = [0.0, 3.0, 7.0, 11.0, 16.0, 20.0];
const YS: [f64; 5] = [0.0, 5.0, 10.0, 15.0, 20.0];
const HEIGHTS: [[f64; 5]; 6] = [
    [3.0, 5.0, 6.0, 5.0, 4.0],
    [5.0, 9.0, 11.0, 8.0, 5.0],
    [6.0, 12.0, 15.0, 10.0, 6.0],
    [4.0, 10.0, 18.0, 9.0, 5.0],
    [5.0, 8.0, 9.0, 7.0, 4.0],
    [2.0, 4.0, 5.0, 4.0, 3.0],
];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let poles = (0..6).map(|i| (0..5).map(|j| p(XS[i], YS[j], HEIGHTS[i][j])).collect()).collect();
    let u = KnotVector::new(vec![0.0, 0.4, 1.0], vec![4, 2, 4], 3);
    let v = KnotVector::new(vec![0.0, 1.0 / 3.0, 1.0], vec![4, 1, 4], 3);
    let top = BSplineSurface::try_new(poles, u, v)?;
    block_under(m, Surface::BSpline(top), 0.0, 20.0)
}
