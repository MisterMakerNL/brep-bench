//! A sheet of two flat faces meeting at a right angle along a crease, the floor (the rectangle in
//! the xy plane from (0, 0, 0) to (10, 8, 0)) and the wall (the rectangle in the yz plane from
//! (0, 0, 0) to (0, 8, 10)), thickened by 1 to the outside of the crease (−z and −x), the moved
//! faces extended until they meet, so the corner stays sharp.
//!
//! cadrs's Thicken of the sheet (the L surface-extruded) gives each face its own slab and leaves
//! the gap between them at the crease open (a sheet whose free edges are not flat loops is not
//! shelled, and slabs are not joined), with no option to extend the moved faces. So the angle is
//! built as the sheet's profile extruded thin instead: the L (10, 0), (0, 0), (0, 10) on the
//! Front plane, a wall 1 thick to its outside, extruded 8 along +y; the thin extrude mitres the
//! outer corner, extending the offset lines until they meet.
use crate::shared::thicken::thin_angle;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    thin_angle(k)
}
