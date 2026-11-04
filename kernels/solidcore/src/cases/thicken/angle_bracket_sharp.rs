//! A sheet of two flat faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) in the xy plane and the wall from (0, 0, 0) to (0, 8, 10) in the yz plane,
//! thickened by 1 to the outside of the crease (to -z and -x), the moved faces extended until
//! they meet: an angle 1 thick.
use crate::shared::thicken::bracket;
use solidcore_ops::{thicken, Body, Result};

pub fn build() -> Result<Body> {
    let sheet = bracket()?;
    // its normals face out of the crease; ops's thicken extends the parted faces until they meet
    thicken(&sheet, 1.0, false)?;
    Ok(sheet)
}
