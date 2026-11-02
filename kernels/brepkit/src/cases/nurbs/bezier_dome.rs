//! A block on the square 0 ≤ x, y ≤ 20 under a bicubic Bézier patch: the patch is brepkit's NURBS
//! surface on the case's sixteen control points (degree 3 each way on the Bézier knots), its top
//! face bounded by the four cubic curves on its boundary control points; each upright side is the
//! flat face between the floor and one of those curves. The six faces are sewn into the solid.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{bezier_knots, block_under_spline};
use crate::solids::p;
use crate::Refusal;

/// z of the control point over (20i/3, 20j/3): row i, column j.
const HEIGHTS: [[f64; 4]; 4] = [
    [4.0, 6.0, 7.0, 5.0],
    [6.0, 14.0, 12.0, 7.0],
    [5.0, 13.0, 16.0, 8.0],
    [3.0, 6.0, 8.0, 6.0],
];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let points = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| p(20.0 * i as f64 / 3.0, 20.0 * j as f64 / 3.0, HEIGHTS[i][j]))
                .collect()
        })
        .collect();
    block_under_spline(topo, (3, 3), &bezier_knots(3), &bezier_knots(3), points)
}
