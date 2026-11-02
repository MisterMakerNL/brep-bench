//! A sheet of two flat faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10), thickened by 1 to the outside of the
//! crease (−z and −x), the corner rounded by the quarter cylinder of radius 1 about the crease.
//!
//! brepkit's thicken takes one face, and its offsets join moved faces only sharp, so the sheet of
//! two is thickened as two faces of the box they bound, from the origin to (10, 8, 10), by
//! brepkit's shell grown outward (the box's four other faces its openings), which extends the
//! moved faces until they meet along (−1, y, −1); that edge is then rounded to radius 1, the
//! round lying 1 from both moved faces, on the cylinder of radius 1 about the crease.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::thicken_faces;
use crate::solids::{edges_where, faces_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 10.0, 8.0, 10.0)?;
    let sheet = faces_where(topo, block, |q| near(q.z(), 0.0) || near(q.x(), 0.0))?;
    let angle = thicken_faces(topo, block, &sheet, 1.0)?;
    let corner = edges_where(topo, angle, |q| near(q.x(), -1.0) && near(q.z(), -1.0))?;
    Ok(all_blended(fillet_v2(topo, angle, &corner, 1.0)?)?)
}
