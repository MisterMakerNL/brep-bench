//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, its four sides upright, its top the
//! clamped bicubic B-spline of 6 × 5 control points (xᵢ, yⱼ, zᵢⱼ), with a double knot at 2/5
//! along u: the surface's face, the floor and the four sides (each bounded by the floor's edge
//! and the cubic B-spline the surface's edge draws in its plane), sewn into a solid.
use crate::shared::nurbs::graph_block;
use crate::Refusal;
use solidcore_ops::Body;

const XS: [f64; 6] = [0.0, 3.0, 7.0, 11.0, 16.0, 20.0];
const YS: [f64; 5] = [0.0, 5.0, 10.0, 15.0, 20.0];
const U_KNOTS: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0];
const V_KNOTS: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, 1.0, 1.0, 1.0];
/// The heights zᵢⱼ, row i (x = xᵢ), column j (y = yⱼ).
const HEIGHTS: [[f64; 5]; 6] = [
    [3.0, 5.0, 6.0, 5.0, 4.0],
    [5.0, 9.0, 11.0, 8.0, 5.0],
    [6.0, 12.0, 15.0, 10.0, 6.0],
    [4.0, 10.0, 18.0, 9.0, 5.0],
    [5.0, 8.0, 9.0, 7.0, 4.0],
    [2.0, 4.0, 5.0, 4.0, 3.0],
];

pub fn build() -> Result<Body, Refusal> {
    let heights: Vec<Vec<f64>> = HEIGHTS.iter().map(|r| r.to_vec()).collect();
    graph_block(&XS, &YS, &heights, &U_KNOTS, &V_KNOTS)
}
