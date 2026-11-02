//! The box from the origin to (20, 14, 10), its four vertical edges rounded to radius 2 with
//! `fillet_v2`, shelled inward to a wall 1 thick, open at its top: brepkit's `shell` moves each
//! round to the cylinder of radius 1 on its axis, a tray whose inside corners are concentric with
//! its outside ones.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{face_through, rounded_plate};
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let tray = rounded_plate(topo)?;
    let top = face_through(topo, tray, p(10.0, 7.0, 10.0))?;
    Ok(shell(topo, tray, 1.0, &[top])?)
}
