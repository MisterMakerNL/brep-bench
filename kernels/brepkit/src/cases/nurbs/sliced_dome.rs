//! The block under the bicubic Bézier dome of `nurbs/bezier-dome` (the case's sixteen control
//! points as brepkit's NURBS surface, its top face bounded by its four boundary cubics, the sides
//! flat faces down to the floor, all sewn), less everything above the plane z = 4 + x/2: a box
//! whose floor lies in that plane (a box turned about the y axis by the plane's slope and set down
//! through the corner (0, 0, 4)), reaching past the block everywhere else.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{bezier_knots, block_under_spline};
use crate::solids::{cut, p, placed};
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
    // a box on the plane z = 0 from x = -20 to 60, y = -10 to 30, up to 40; turned about the y
    // axis so its floor rises 1 in 2 along x (a right-handed turn about y by −atan(1/2) tips +x
    // up), then lifted so its floor passes through (0, 0, 4)
    let knife = make_box(topo, 80.0, 40.0, 40.0)?;
    let tilt = -(0.5f64).atan();
    let knife = placed(
        topo,
        knife,
        Mat4::translation(0.0, 0.0, 4.0)
            * Mat4::rotation_y(tilt)
            * Mat4::translation(-20.0, -10.0, 0.0),
    )?;
    Ok(cut(topo, block, knife)?)
}
