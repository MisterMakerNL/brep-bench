//! A capsule's end, the cylinder of radius 5 on the z axis from z = 0 to z = 10 fused with the ball
//! of radius 5 about (0, 0, 10), shelled inward to a wall 1 thick, open at its base: brepkit's
//! `shell` moves the side to the cylinder of radius 4 and the dome to the sphere of radius 4, still
//! meeting tangentially at z = 10, with an annular rim at z = 0.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{capsule_end, face_through};
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let capsule = capsule_end(topo)?;
    let base = face_through(topo, capsule, p(0.0, 0.0, 0.0))?;
    Ok(shell(topo, capsule, 1.0, &[base])?)
}
