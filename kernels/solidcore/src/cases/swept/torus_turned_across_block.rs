//! A curved block, the sector of the ring between radii 8 and 12 about the z axis, z from −1 to 1
//! and polar angle from 0° to 60°, grooved by a cutter, the torus of radii 2.5 and 2 centred on
//! (5, 0, 0) with its axis leaning 25° from z toward +y, turned about the z axis from −140° to
//! +140°.
//!
//! Refused. solidcore has no sweep of a solid. Its rotational sweep turns a profile of curves
//! about an axis, but the cutter's axis is skew to the turn's, so its sweep is no profile's
//! turned: the groove's wall is the envelope of the torus under the turn, a curve on it that is
//! none of its circles, which the kernel does not compute; fitted to points here, it would be
//! geometry made outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore has no sweep of a solid, and its rotational sweep turns a profile of curves: \
         the cutter leans across the turn, so the groove's wall is the envelope of the torus \
         along a curve on it that is none of its circles, which it cannot build"
        .into())
}
