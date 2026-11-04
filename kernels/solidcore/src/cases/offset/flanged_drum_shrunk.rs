//! A drum, the cylinder of radius 8 on the z axis from z = 0 to z = 6, with a flange round its
//! side from radius 8 out to 11 and from z = 2.2 to z = 3.8, shrunk by 1: every point at least 1
//! from its outside. The flange, thinner than 2, is gone but for a low ridge where it met the
//! drum, the kernel's offset inward, rounded where the moved faces part.
use crate::bench::fuse;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let drum = make_cylinder([0.0, 0.0, 0.0], 8.0, 6.0)?;
    let flange = make_cylinder([0.0, 0.0, 2.2], 11.0, 1.6)?;
    let flanged = fuse(drum, flange)?;
    offset_rounded(&flanged, -1.0)
}
