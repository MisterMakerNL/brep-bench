//! A wheel, the disk of radius 40 about the vertical line through (−50, 0) from z = −8 to −2,
//! cut by a plus (arms 30 long and 8 wide along ±x and ±y, from z = −10 to 0) turning about the
//! z axis by t while the wheel turns by −2t/3 about its own axis, over t from 0° to 540°.
//!
//! Refused. solidcore has no sweep of a solid, and this one cannot be built from its sweeps of
//! curves: seen from the wheel the plus rolls in its plane, so the cut is the region it sweeps
//! there, bounded by the envelopes of its sides under the roll and the trochoids its corners
//! trace, curves the kernel has no form of and does not compute. Fitted to points of the
//! envelope here, they would be geometry made outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore has no sweep of a solid, and its sweeps carry curves along lines, arcs, paths \
         and screws: the wheel's flanks are the envelope of the plus's sides as it rolls in mesh \
         with the wheel and the trochoids its corners trace, which it cannot build"
        .into())
}
