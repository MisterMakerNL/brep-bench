//! Two balls of radius 5, centred on the origin and on (6, 0, 0): the lens common to both.
use crate::bench::common;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let first = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let second = make_sphere([6.0, 0.0, 0.0], 5.0)?;
    common(first, second)
}
