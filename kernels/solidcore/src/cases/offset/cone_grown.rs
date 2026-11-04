//! A cone on the z axis, its base the disc of radius 3 at z = 0 and its apex at (0, 0, 4), grown
//! by 1: every point within 1 of it, its rim a piece of a torus's tube and its apex a cap of a
//! ball, the kernel's offset with its convex edges rounded.
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let cone = make_cone([0.0, 0.0, 0.0], 3.0, 0.0, 4.0)?;
    offset_rounded(&cone, 1.0)
}
