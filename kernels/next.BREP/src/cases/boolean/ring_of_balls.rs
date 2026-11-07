//! Twelve balls of radius 3, their centres evenly spaced on the circle of radius 10 about the z axis
//! from (10, 0, 0), joined one after another.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = |i: usize| {
        let (s, c) = (TAU * i as f64 / 12.0).sin_cos();
        make_sphere_brep(p(10.0 * c, 10.0 * s, 0.0), 3.0, p(0.0, 0.0, 1.0))
    };
    let mut ring = ball(0)?;
    for i in 1..12 {
        ring = union(&ring, &ball(i)?)?;
    }
    Ok(ring)
}
