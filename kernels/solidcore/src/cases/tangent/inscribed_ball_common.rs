//! A cube of side 10 between the origin and (10, 10, 10), and its inscribed ball, of radius 5
//! about (5, 5, 5), touching each face at its middle: the solid common to both, the ball.
use crate::bench::common;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let ball = make_sphere([5.0, 5.0, 5.0], 5.0)?;
    common(cube, ball)
}
