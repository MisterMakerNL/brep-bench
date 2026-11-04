//! A ball of radius 5 centred on the origin hollowed to a wall 1 thick with no opening: the ball
//! less the ball of radius 4, one solid bounded by two spheres. A shell with no face opened.
use crate::bench::order;
use crate::Refusal;
use solidcore_ops::{make_sphere, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    shell(&ball, &order(&ball)?, &[], -1.0)?;
    Ok(ball)
}
