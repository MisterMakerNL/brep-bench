//! A block on the square 0 ≤ x, y ≤ 20 under a Bézier patch of degree 15 each way: brepkit's NURBS
//! surface on the case's 16 × 16 control points (on the Bézier knots of degree 15), its top face
//! bounded by the four degree-15 curves on its boundary control points; each upright side is the
//! flat face between the floor and one of those curves. The six faces are sewn into the solid.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{bezier_knots, block_under_spline};
use crate::solids::p;
use crate::Refusal;

const DEGREE: usize = 15;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let n = DEGREE as f64;
    // Pᵢⱼ = (20i/15, 20j/15, 4 + ((3i + 5j) mod 7))
    let points = (0..=DEGREE)
        .map(|i| {
            (0..=DEGREE)
                .map(|j| {
                    p(
                        20.0 * i as f64 / n,
                        20.0 * j as f64 / n,
                        4.0 + ((3 * i + 5 * j) % 7) as f64,
                    )
                })
                .collect()
        })
        .collect();
    block_under_spline(
        topo,
        (DEGREE, DEGREE),
        &bezier_knots(DEGREE),
        &bezier_knots(DEGREE),
        points,
    )
}
