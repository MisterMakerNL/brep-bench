//! A ball of radius 5 centred on the origin, with a hole of radius 3 bored right through it along
//! the z axis: a cylinder running past both ends of the bore, cut from the ball.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let bore = make_cylinder([0.0, 0.0, -6.0], 3.0, 12.0)?;
    cut(ball, bore)
}
