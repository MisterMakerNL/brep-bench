//! A cube of side 10 from the origin to (10, 10, 10), and on it a second from (5, 0, 10) to
//! (15, 10, 20), overhanging by half: the two joined into one solid.
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let first = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let second = make_box([5.0, 0.0, 10.0], [10.0, 10.0, 10.0])?;
    fuse(first, second)
}
