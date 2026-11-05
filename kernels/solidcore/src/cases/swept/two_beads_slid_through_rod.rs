//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of
//! two beads slid past it: the tori on one axis parallel to y through (11.5, 0, −9), the first
//! centred there with radii 6 and 2, the second 1.5 along the axis with radii 6 and 1.5, slid
//! together without turning 40 up the z axis, cut from the rod.
//!
//! solidcore has no sweep of a solid, so each bead's sweep is built as what it is: the torus at
//! each end and the stadium it covers seen along the slide, extruded along it
//! ([`torus_slid`]). The wheel's sweep is the union of its beads', and the rod less it is the
//! rod less each piece.
use crate::bench::cut_all;
use crate::shared::swept::{rod, torus_slid};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let first = torus_slid([11.5, 0.0, -9.0], 6.0, 2.0, 40.0)?;
    let second = torus_slid([11.5, 1.5, -9.0], 6.0, 1.5, 40.0)?;
    cut_all(rod()?, first.into_iter().chain(second))
}
