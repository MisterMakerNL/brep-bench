//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its curved side: only the two ends are walls, each moved 1 inward, so the result should
//! be two separate discs 1 thick, at z = 0..1 and z = 9..10. brepkit's `shell` returns one solid,
//! which is returned as it is.
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::face_through;
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cylinder = make_cylinder(topo, 5.0, 10.0)?;
    let side = face_through(topo, cylinder, p(5.0, 0.0, 5.0))?;
    Ok(shell(topo, cylinder, 1.0, &[side])?)
}
