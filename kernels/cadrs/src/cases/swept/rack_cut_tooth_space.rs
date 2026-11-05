//! A gear blank, the disk of radius 22 about the y axis from y = 0 to y = 6, with one tooth space
//! cut by a rack tooth (module 2, 20° pressure angle) rolled on its pitch circle of radius 20
//! from 60° on one side to 60° on the other.
//!
//! Refused. cadrs has no sweep of a solid: its `sweep_body` carries points, wires and sheets
//! one kind higher, only straight along a vector or about one axis. Seen from the blank the rack
//! tooth rolls (turns and slides at once), so the space's flanks are the envelope of the tooth's
//! flanks under that roll, involutes of the base circle, and its root the curve the tooth's tip
//! corners trace: curves the kernel has no form of and does not compute; fitted to points here,
//! they would be geometry made outside the kernel.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no sweep of a solid, and sweeps points, wires and sheets only straight or \
         about one axis: the tooth space is the envelope of the rack tooth rolled on the pitch \
         circle (involute flanks, a trochoidal root), which it cannot build",
    ))
}
