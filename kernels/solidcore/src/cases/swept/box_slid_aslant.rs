//! The solid the box from (0, 0, 0) to (10, 6, 4) fills as it slides along (8, 5, 3) without
//! turning, ending as the box from (8, 5, 3) to (18, 11, 7).
use crate::shared::swept::slid;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0; 3], [10.0, 6.0, 4.0])?;
    slid(&block, [8.0, 5.0, 3.0])
}
