//! A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base and 2 at its top:
//! its section beside the axis, on the Front plane, revolved a whole turn about z (the kernel has
//! no cone primitive).
use crate::bench::{axis, p3, polygon, revolve, sketch, v3, xz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = polygon(&[(0.0, 0.0), (5.0, 0.0), (2.0, 6.0), (0.0, 6.0)]);
    let s = sketch(xz(0.0), vec![section]);
    revolve(k, &s, axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)
}
