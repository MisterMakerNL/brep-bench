//! A napkin ring, the ball of radius 5 centred on the origin with a hole of radius 3 bored through
//! it along z, grown by 1: every point within 1 of it, its sphere moved out to radius 6, its bore
//! in to radius 2, its rims pieces of tori, the kernel's offset with its convex edges rounded.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let bore = make_cylinder([0.0, 0.0, -6.0], 3.0, 12.0)?;
    let ring = cut(ball, bore)?;
    offset_rounded(&ring, 1.0)
}
