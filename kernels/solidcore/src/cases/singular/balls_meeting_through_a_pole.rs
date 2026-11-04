//! Two balls of radius 5, centred on the origin and on (4.8, 0, 3.6), united. solidcore puts each
//! ball's poles on the vertical through its centre, so the circle where they meet runs through
//! a pole of each.
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let first = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let second = make_sphere([4.8, 0.0, 3.6], 5.0)?;
    fuse(first, second)
}
