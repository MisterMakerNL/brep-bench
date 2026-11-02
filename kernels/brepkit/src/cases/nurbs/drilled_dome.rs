//! The block under the bicubic Bézier dome of `nurbs/bezier-dome` (the case's sixteen control
//! points as brepkit's NURBS surface, its top face bounded by its four boundary cubics, the sides
//! flat faces down to the floor, all sewn), drilled through by a cylinder of radius 3 about the line
//! x = 12, y = 8, which runs from below the floor to above the dome.
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{bezier_knots, block_under_spline};
use crate::solids::{cut, moved, p};
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
    let block = block_under_spline(topo, (3, 3), &bezier_knots(3), &bezier_knots(3), points)?;
    let drill = make_cylinder(topo, 3.0, 22.0)?;
    let drill = moved(topo, drill, 12.0, 8.0, -1.0)?;
    Ok(cut(topo, block, drill)?)
}
