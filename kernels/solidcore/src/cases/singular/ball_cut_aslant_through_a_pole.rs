//! A ball of radius 5 centred on the origin, its poles on the z axis (solidcore's own), less the
//! cap beyond the plane x + z = 5, which passes through its pole at (0, 0, 5): the half-space is
//! the block beyond x = 5 tilted 45° about the line through (5, 0, 0) along y, until its face's
//! normal is (1, 0, 1)/√2.
use crate::bench::cut;
use crate::shared::singular::beyond;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    // turning +x about y by -45° takes it to (1, 0, 1)/√2
    cut(ball, beyond(5.0, -45.0)?)
}
