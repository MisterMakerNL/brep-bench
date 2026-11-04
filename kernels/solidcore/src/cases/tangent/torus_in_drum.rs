//! A torus about the z axis, of radii 10 (its tube's middle circle) and 3 (its tube), united
//! with a drum of radius 13 on the z axis from z = -1 to z = 1, whose wall the torus touches
//! from inside along its outer equator.
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    let torus = make_torus([0.0, 0.0, 0.0], 10.0, 3.0)?;
    let drum = make_cylinder([0.0, 0.0, -1.0], 13.0, 2.0)?;
    fuse(torus, drum)
}
