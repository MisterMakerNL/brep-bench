//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel slid
//! past it: the torus of radii 6 and 2, its axis parallel to y, slid without turning from centre
//! (11.5, 0, −9) up to (11.5, 0, 29), cut from the rod.
//!
//! solidcore has no sweep of a solid, so the wheel's sweep is built as what it is: the torus at
//! each end and the stadium it covers seen along the slide, extruded along it
//! ([`torus_slid`]). The rod less their union is the rod less each.
use crate::bench::cut_all;
use crate::shared::swept::{rod, torus_slid};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    cut_all(rod()?, torus_slid([11.5, 0.0, -9.0], 6.0, 2.0, 38.0)?)
}
