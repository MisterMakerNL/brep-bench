//! A block from (-10, -10, -10) to (10, 10, 0) less a ball of radius 5 centred on the origin: a
//! hemispherical pocket in its top face.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-10.0, -10.0, -10.0], [20.0, 20.0, 10.0])?;
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    cut(block, ball)
}
