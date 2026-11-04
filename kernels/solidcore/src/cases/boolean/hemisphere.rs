//! A ball of radius 5 centred on the origin, with everything below the xy plane cut away: a
//! block under z = 0 reaching past the ball.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let below = make_box([-10.0, -10.0, -10.0], [20.0, 20.0, 10.0])?;
    cut(ball, below)
}
