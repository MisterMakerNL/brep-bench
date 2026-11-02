//! A block on the square 0 ≤ x, y ≤ 20 under a clamped bicubic B-spline: brepkit's NURBS surface on
//! the case's 6 × 5 control points and its two knot vectors (the double knot at 2/5 along u), its
//! top face bounded by the four cubic B-splines on its boundary control points, on the same knots;
//! each upright side is the flat face between the floor and one of those curves. The six faces are
//! sewn into the solid.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::block_under_spline;
use crate::solids::p;
use crate::Refusal;

const KNOTS_U: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0];
const KNOTS_V: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, 1.0, 1.0, 1.0];
const XS: [f64; 6] = [0.0, 3.0, 7.0, 11.0, 16.0, 20.0];
const YS: [f64; 5] = [0.0, 5.0, 10.0, 15.0, 20.0];
/// z of the control point (xᵢ, yⱼ): row i, column j.
const HEIGHTS: [[f64; 5]; 6] = [
    [3.0, 5.0, 6.0, 5.0, 4.0],
    [5.0, 9.0, 11.0, 8.0, 5.0],
    [6.0, 12.0, 15.0, 10.0, 6.0],
    [4.0, 10.0, 18.0, 9.0, 5.0],
    [5.0, 8.0, 9.0, 7.0, 4.0],
    [2.0, 4.0, 5.0, 4.0, 3.0],
];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let points = (0..6)
        .map(|i| (0..5).map(|j| p(XS[i], YS[j], HEIGHTS[i][j])).collect())
        .collect();
    block_under_spline(topo, (3, 3), &KNOTS_U, &KNOTS_V, points)
}
