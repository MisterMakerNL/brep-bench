//! A dumbbell-shaped plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6
//! up, shelled inward to a wall 1.5 thick, open at its top, the wall every point within 1.5 of
//! its faces other than the top: the neck, narrower than twice the wall, is left solid, and each
//! pocket bulges towards the neck in arcs about the neck's mouth corners.
//!
//! geop's shell moves each face and meets the moved faces sharp: it has no shell whose wall is
//! every point within the thickness of the faces, rounding the wall at a concave edge, nor one
//! that fills where moved walls cross. So it is refused.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop's shell meets moved faces sharp and has no shell whose wall is every point within \
         the thickness of the faces (rounding the wall about a concave edge); the neck's moved \
         walls cross, and the pockets' rounded mouths are no moved face"
        .into())
}
