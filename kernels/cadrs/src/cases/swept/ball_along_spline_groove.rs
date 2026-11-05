//! A block, the box from (−6, −6, −8) to (36, 26, 0), grooved by a ball of radius 3 whose center
//! follows the cubic Bézier curve in the top face with control points (0, 0, 0), (10, 0, 0),
//! (20, 20, 0) and (30, 20, 0): everything the ball passes through cut from the block.
//!
//! cadrs has no sweep of a solid, so the ball's sweep is built as what it is: the tube about
//! its center's path, the disk of radius 3 on the Right plane (square to the curve where it
//! starts, along +x) swept along the curve drawn on the Top plane, and the ball at each end of
//! the curve, whose half beyond the tube closes it. The kernel has no sphere primitive: each
//! ball is a half disc revolved.
use crate::bench::{ball, block, cut_each, p2, p3, round, sketch, xy, yz};
use crate::shared::sweep::{on, sweep};
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = block(k, p3(-6.0, -6.0, -8.0), p3(36.0, 26.0, 0.0))?;
    let curve = Curve2::Bezier {
        poles: [p2(0.0, 0.0), p2(10.0, 0.0), p2(20.0, 20.0), p2(30.0, 20.0)],
        source: None,
    };
    let disk = sketch(yz(0.0), vec![round((0.0, 0.0), 3.0)]);
    let tube = sweep(k, &disk, vec![on(xy(0.0), curve)])?;
    let first = ball(k, p3(0.0, 0.0, 0.0), 3.0)?;
    let last = ball(k, p3(30.0, 20.0, 0.0), 3.0)?;
    cut_each(k, body, &[tube, first, last])
}
