//! A stepped shaft on the z axis, one solid: a cylinder of radius 5 from z = 0 to 10, and on top
//! of it one of radius 3 from z = 10 to 20.
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let lower = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    let upper = make_cylinder([0.0, 0.0, 10.0], 3.0, 10.0)?;
    fuse(lower, upper)
}
