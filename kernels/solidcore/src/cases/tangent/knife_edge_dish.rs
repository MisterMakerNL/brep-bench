//! A cylinder of radius 5 on the z axis from z = 0 to 10, less a ball of radius 5 centred on the
//! middle of its top face, (0, 0, 10): a hemispherical dish meeting the wall in a knife edge.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let rod = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    let ball = make_sphere([0.0, 0.0, 10.0], 5.0)?;
    cut(rod, ball)
}
