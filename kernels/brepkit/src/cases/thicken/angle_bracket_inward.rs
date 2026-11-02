//! A sheet of two flat faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10), thickened by 1 to the inside of the
//! crease (+z and +x), each moved face trimmed where it meets the other: an L 1 thick.
//!
//! brepkit's thicken takes one face, so the sheet of two is thickened as two faces of the box
//! they bound, from the origin to (10, 8, 10), by brepkit's shell inward, the box's four other
//! faces its openings.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::thicken_faces;
use crate::solids::{faces_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 10.0, 8.0, 10.0)?;
    let sheet = faces_where(topo, block, |q| near(q.z(), 0.0) || near(q.x(), 0.0))?;
    Ok(thicken_faces(topo, block, &sheet, -1.0)?)
}
