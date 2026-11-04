//! A block, the box from (-6, -6, -8) to (36, 26, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre follows the cubic Bézier curve in the top face with control
//! points (0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0): everything the ball passes through,
//! cut from the block.
use crate::bench::{circle, cut, fuse_all};
use crate::Refusal;
use solidcore_ops::sweep::{sweep_placed, Curve, SweepFrame};
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-6.0, -6.0, -8.0], [42.0, 32.0, 8.0])?;
    let path = Curve::Nurbs {
        degree: 3,
        points: vec![
            [0.0, 0.0, 0.0, 1.0],
            [10.0, 0.0, 0.0, 1.0],
            [20.0, 20.0, 0.0, 1.0],
            [30.0, 20.0, 0.0, 1.0],
        ],
        knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
    };
    // The kernel sweeps a whole body only along a line or about an axis (`shared::swept`), not
    // along a curve, so the ball's sweep is built as elementary geometry gives it: the tube of
    // the ball's radius about its centre's path, the disc square to the path (its tangent at the
    // start is +x) swept along it turning with it, with the ball at each end.
    let disc = circle([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 3.0);
    let (tube, _, _) = sweep_placed(&[disc], &[path], SweepFrame::Follow, true)?;
    let start = make_sphere([0.0; 3], 3.0)?;
    let end = make_sphere([30.0, 20.0, 0.0], 3.0)?;
    let groove = fuse_all([tube, start, end])?;
    cut(block, groove)
}
