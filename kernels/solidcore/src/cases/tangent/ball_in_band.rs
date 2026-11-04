//! A ball of radius 5 centred on the origin, united with a cylinder of the same radius on the z
//! axis from z = -3 to z = 3, whose wall the sphere touches from inside along its equator.
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let band = make_cylinder([0.0, 0.0, -3.0], 5.0, 6.0)?;
    fuse(ball, band)
}
