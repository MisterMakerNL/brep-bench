//! A block, the box from (−6, −6, −8) to (18, 18, 0), grooved by a ball of radius 3 whose
//! center swings a quarter turn about the z axis in the top face, from (12, 0, 0) to (0, 12, 0):
//! everything the ball passes through cut from the block.
//!
//! cadrs has no sweep of a solid, so the ball's sweep is built as what it is: the tube about
//! its center's path, the disk of radius 3 about (12, 0, 0) on the Front plane (square to the
//! arc there) revolved the quarter turn about the z axis, and the ball at each end of the path,
//! whose half beyond the tube closes it. Each ball is cadrs's sphere primitive.
use crate::bench::{axis, ball, block, cut_each, p3, revolve, round, sketch, v3, xz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = block(k, p3(-6.0, -6.0, -8.0), p3(18.0, 18.0, 0.0))?;
    let disk = sketch(xz(0.0), vec![round((12.0, 0.0), 3.0)]);
    let tube = revolve(k, &disk, axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), Some(90.0))?;
    let first = ball(k, p3(12.0, 0.0, 0.0), 3.0)?;
    let last = ball(k, p3(0.0, 12.0, 0.0), 3.0)?;
    cut_each(k, body, &[tube, first, last])
}
