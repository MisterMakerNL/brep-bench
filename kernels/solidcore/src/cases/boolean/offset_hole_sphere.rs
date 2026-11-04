//! A ball of radius 20 centred on the origin, bored right through parallel to the y axis by a
//! hole of radius 6 whose axis is the line x = 5, z = 0: a cylinder running past the ball at
//! both ends, cut from it.
use crate::bench::cut;
use crate::shared::boolean::cylinder_along_y;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 20.0)?;
    let bore = cylinder_along_y([5.0, -25.0, 0.0], 6.0, 50.0)?;
    cut(ball, bore)
}
