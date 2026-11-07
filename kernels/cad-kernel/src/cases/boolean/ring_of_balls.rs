//! Twelve balls of radius 3, their centres evenly spaced on the circle of radius 10 about z from
//! (10, 0, 0), joined one after another.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

pub fn build() -> Result<Brep, String> {
    let ball = |k: usize| {
        let (s, c) = (TAU * k as f64 / 12.).sin_cos();
        sphere([10. * c, 10. * s, 0.], [0., 0., 1.], 3.)
    };
    let mut ring = ball(0)?;
    for k in 1..12 {
        ring = combined(&ring, &ball(k)?, Op::Union, 0.)?;
    }
    Ok(ring)
}
