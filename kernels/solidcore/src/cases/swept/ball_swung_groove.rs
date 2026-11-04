//! A block, the box from (-6, -6, -8) to (18, 18, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre swings a quarter turn about the z axis from (12, 0, 0) to
//! (0, 12, 0): everything the ball passes through, cut from the block.
use crate::bench::cut;
use crate::shared::swept::swung;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-6.0, -6.0, -8.0], [24.0, 24.0, 8.0])?;
    let ball = make_sphere([12.0, 0.0, 0.0], 3.0)?;
    let groove = swung(&ball, [0.0; 3], [0.0, 0.0, 1.0], 90.0)?;
    cut(block, groove)
}
