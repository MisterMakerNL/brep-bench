//! A sheet of two flat faces meeting at a right angle along a crease, the floor (the rectangle in
//! the xy plane from (0, 0, 0) to (10, 8, 0)) and the wall (the rectangle in the yz plane from
//! (0, 0, 0) to (0, 8, 10)), thickened by 1 to the outside of the crease (−z and −x), the gap the
//! moved faces leave at the crease closed by the quarter cylinder of radius 1 about it.
//!
//! cadrs's Thicken of the sheet (the L surface-extruded) gives each face its own slab and leaves
//! the gap between them at the crease open (a sheet whose free edges are not flat loops is not
//! shelled, and slabs are not joined). So the angle is built as the sheet's profile extruded thin
//! instead (the L (10, 0), (0, 0), (0, 10) on the Front plane, a wall 1 thick to its outside,
//! extruded 8 along +y, its outer corner mitred along (−1, y, −1)), and that corner edge is
//! filleted to radius 1: the round lies 1 from both moved faces, on the cylinder of radius 1
//! about the crease.
use crate::bench::{edge_at, p3};
use crate::shared::thicken::thin_angle;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let angle = thin_angle(k)?;
    let corner = edge_at(k, angle, p3(-1.0, 4.0, -1.0))?;
    let r = k.fillet(angle, &[corner], 1.0)?;
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the fillet made no body"))
}
