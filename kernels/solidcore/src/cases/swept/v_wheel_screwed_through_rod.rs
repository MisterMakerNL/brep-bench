//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel with
//! a sharp 60° V rim (a triangle turned about an axis through (14, 0, 0) leaning by the helix
//! angle) carried along the screw about the z axis rising 30 a turn, from a turn of −144° to
//! +384°.
//!
//! Refused. solidcore has no sweep of a solid. Its screw sweep carries a profile of curves along
//! a screw, but the wheel's sweep is no profile's: the groove's flanks are the envelopes of the
//! wheel's two cones under the screw, at each instant the curve on each cone where the screw's
//! motion is tangent to it, which the kernel does not compute; fitted to points here, they would
//! be geometry made outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore has no sweep of a solid, and its screw sweep carries a profile of curves: the \
         groove's flanks are the envelopes of the V wheel's cones under the screw, which it \
         cannot build"
        .into())
}
