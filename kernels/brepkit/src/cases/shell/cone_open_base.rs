//! A solid cone on the z axis, its base the disc of radius 6 at z = 0 and its apex at (0, 0, 8),
//! shelled inward to a wall 1 thick, open at its base: brepkit's `shell` moves the cone's side 1
//! inward, to the cone of the same half-angle whose apex is 5/3 lower, and the rim is the annulus
//! it leaves on the base's plane.
use brepkit_operations::primitives::make_cone;
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::face_through;
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cone = make_cone(topo, 6.0, 0.0, 8.0)?;
    let base = face_through(topo, cone, p(0.0, 0.0, 0.0))?;
    Ok(shell(topo, cone, 1.0, &[base])?)
}
