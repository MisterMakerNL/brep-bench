//! A gear blank, the disk of radius 22 about the y axis from y = 0 to y = 6, with one tooth space
//! cut by a rack tooth (module 2, 20° pressure angle) whose pitch line rolls on the blank's
//! pitch circle of radius 20, from 60° on one side to 60° on the other.
//!
//! Refused. solidcore has no sweep of a solid, and this one cannot be built from its sweeps of
//! curves: seen from the blank the tooth rolls in the blank's plane, and the space's flanks are
//! the envelope of the tooth's flanks under the roll (involutes of the base circle) and its
//! root the curve the tip's corners trace, curves the kernel has no form of and does not
//! compute. Fitted to points here, they would be geometry made outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore has no sweep of a solid, and its sweeps carry curves along lines, arcs, paths \
         and screws: the tooth space's flanks are the envelope of the rack tooth rolled on the \
         pitch circle (involutes) and its root the trace of the tip's corners, which it cannot \
         build"
        .into())
}
