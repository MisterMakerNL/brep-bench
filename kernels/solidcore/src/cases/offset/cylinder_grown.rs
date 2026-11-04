//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1: every point within 1 of
//! it, its rims the outer quarters of tori, the kernel's offset with its convex edges rounded.
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let drum = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    offset_rounded(&drum, 1.0)
}
