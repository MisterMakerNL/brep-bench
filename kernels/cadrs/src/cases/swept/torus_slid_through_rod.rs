//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel slid
//! past it: the torus of radii 6 and 2, its axis parallel to y, slid without turning from center
//! (11.5, 0, −9) up to (11.5, 0, 29), cut from the rod.
//!
//! cadrs has no sweep of a solid, so the wheel's sweep is built as what it is
//! ([`torus_slid`]): the torus at each end and the stadium it covers seen along the slide,
//! extruded along it. The kernel has no torus primitive: each torus is a circle revolved.
use crate::bench::{cut_each, p3};
use crate::shared::swept::{rod, torus_slid};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = rod(k)?;
    let wheel = torus_slid(k, p3(11.5, 0.0, -9.0), 6.0, 2.0, 38.0)?;
    cut_each(k, body, &wheel)
}
