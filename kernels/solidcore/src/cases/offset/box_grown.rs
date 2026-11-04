//! The box from the origin to (10, 8, 6) grown by 1: every point within 1 of it, its faces moved
//! out, its edges quarter cylinders and its corners eighths of a ball, the kernel's offset with
//! its convex edges rounded.
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    offset_rounded(&block, 1.0)
}
