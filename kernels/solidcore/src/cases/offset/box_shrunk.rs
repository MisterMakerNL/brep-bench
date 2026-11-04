//! The box from the origin to (10, 8, 6) shrunk by 1: every point at least 1 from its outside,
//! the box from (1, 1, 1) to (9, 7, 5), the kernel's offset inward.
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    offset_rounded(&block, -1.0)
}
