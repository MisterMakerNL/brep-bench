//! A drum, the cylinder of radius 10 on the z axis from z = 0 to z = 10, with a groove round its
//! side from z = 4.2 to z = 5.8 cut to radius 7, grown by 1: every point within 1 of it. The
//! groove, narrower than 2, fills but for a shallow notch, the kernel's offset with its convex
//! edges rounded.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let drum = make_cylinder([0.0, 0.0, 0.0], 10.0, 10.0)?;
    // the groove: a ring from radius 7 out past the side
    let ring = cut(make_cylinder([0.0, 0.0, 4.2], 11.0, 1.6)?, make_cylinder([0.0, 0.0, 4.0], 7.0, 2.0)?)?;
    let grooved = cut(drum, ring)?;
    offset_rounded(&grooved, 1.0)
}
