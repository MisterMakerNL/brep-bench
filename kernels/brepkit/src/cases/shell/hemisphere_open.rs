//! The lower half of the ball of radius 5 about the origin (the ball less a box covering z > 0),
//! shelled inward to a wall 1 thick, open at its flat face: brepkit's `shell` moves the sphere to
//! the sphere of radius 4, a bowl with an annular rim at z = 0.
use brepkit_operations::primitives::{make_box, make_sphere};
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::face_through;
use crate::solids::{cut, moved, p};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let ball = make_sphere(topo, 5.0, 64)?;
    let above = make_box(topo, 12.0, 12.0, 6.0)?;
    let above = moved(topo, above, -6.0, -6.0, 0.0)?;
    let half = cut(topo, ball, above)?;
    let top = face_through(topo, half, p(0.0, 0.0, 0.0))?;
    Ok(shell(topo, half, 1.0, &[top])?)
}
