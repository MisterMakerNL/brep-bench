//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of
//! two beads slid past it: tori on one axis parallel to y, the first centered (11.5, 0, −9) of
//! radii 6 and 2, the second centered (11.5, 1.5, −9) of radii 6 and 1.5, their union slid
//! without turning 40 up the z axis, cut from the rod.
//!
//! cadrs's sweep of a solid (`Kernel::sweep_solid`) meshes what a torus fills as it slides, so
//! the wheel's sweep is built as what it is: the union of
//! each bead's sweep ([`torus_slid`]: the torus at each end and the stadium it covers seen along
//! the slide, extruded along it). Each torus is cadrs's torus primitive.
use crate::bench::{cut_each, p3};
use crate::shared::swept::{rod, torus_slid};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = rod(k)?;
    let mut wheel = torus_slid(k, p3(11.5, 0.0, -9.0), 6.0, 2.0, 40.0)?;
    wheel.extend(torus_slid(k, p3(11.5, 1.5, -9.0), 6.0, 1.5, 40.0)?);
    cut_each(k, body, &wheel)
}
