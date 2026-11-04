//! Two cubes of side 10 side by side, from the origin to (10, 10, 10) and from (10, 0, 0) to
//! (20, 10, 10), joined into one solid: the face they share goes.
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let first = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let second = make_box([10.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    fuse(first, second)
}
