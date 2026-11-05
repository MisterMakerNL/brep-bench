//! A cube of side 10 between the origin and (10, 10, 10), united with the ball of radius 5 at
//! its center, which touches each of its faces at one point: the union is the cube. The ball is
//! a half disc revolved (the kernel has no sphere primitive).
use crate::bench::{ball, block, p3, union};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let b = ball(k, p3(5.0, 5.0, 5.0), 5.0)?;
    union(k, cube, b)
}
