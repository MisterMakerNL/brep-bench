//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its top: brepkit's `shell` moves the side to the cylinder of radius 4 and the bottom to
//! z = 1, a cup with an annular rim at z = 10.
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::face_through;
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cylinder = make_cylinder(topo, 5.0, 10.0)?;
    let top = face_through(topo, cylinder, p(0.0, 0.0, 10.0))?;
    Ok(shell(topo, cylinder, 1.0, &[top])?)
}
