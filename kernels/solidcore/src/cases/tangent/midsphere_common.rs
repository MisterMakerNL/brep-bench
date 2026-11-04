//! A cube of side 10 from (-5, -5, -5) to (5, 5, 5), and the ball about the origin of radius
//! 5√2, touching each of the cube's edges at its midpoint: the solid common to both.
use crate::bench::common;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([-5.0, -5.0, -5.0], [10.0, 10.0, 10.0])?;
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0 * 2f64.sqrt())?;
    common(cube, ball)
}
