//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel,
//! the torus of radii 6 and 2 centred on (11.5, 0, 0) with its axis leaning by the helix angle,
//! carried along the screw about the z axis rising 30 a turn, from a turn of −108° to +348°.
//!
//! Refused. solidcore has no sweep of a solid. Its screw sweep carries a profile of curves along
//! a screw, but the wheel's sweep is no profile's: its boundary is the envelope of the torus
//! under the screw, at each instant the curve on the torus where the screw's motion is tangent
//! to it, which is none of the torus's own circles. The kernel does not compute that curve;
//! fitted to points here, it would be geometry made outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore has no sweep of a solid, and its screw sweep carries a profile of curves: the \
         groove's wall is the envelope of the torus under the screw, along a curve on it that is \
         none of its circles, which it cannot build"
        .into())
}
