//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, its four sides upright, its top the
//! Bézier patch of degree 15 each way whose control point Pᵢⱼ is (20i/15, 20j/15, zᵢⱼ),
//! zᵢⱼ = 4 + ((3i + 5j) mod 7): the patch's face, the floor and the four sides (each bounded by
//! the floor's edge and the curve of degree 15 the patch's edge draws in its plane), sewn.
use crate::shared::nurbs::{bezier_knots, graph_block};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let grid: Vec<f64> = (0..16).map(|i| 20.0 * i as f64 / 15.0).collect();
    let heights: Vec<Vec<f64>> =
        (0..16).map(|i| (0..16).map(|j| 4.0 + ((3 * i + 5 * j) % 7) as f64).collect()).collect();
    graph_block(&grid, &grid, &heights, &bezier_knots(15), &bezier_knots(15))
}
