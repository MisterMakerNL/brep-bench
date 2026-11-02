//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top, its
//! edges kept sharp. brepkit's `shell_v2` takes a signed thickness, positive for a wall grown
//! outside the faces; its moved faces are extended until they meet, and the open top's rim lies
//! on its plane.
use brepkit_operations::offset_v2::shell_v2;
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::face_through;
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 10.0, 8.0, 6.0)?;
    let top = face_through(topo, block, p(5.0, 4.0, 6.0))?;
    Ok(shell_v2(topo, block, 1.0, &[top])?)
}
