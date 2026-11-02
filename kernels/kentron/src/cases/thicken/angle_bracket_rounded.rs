//! A creased sheet, the floor from (0, 0, 0) to (10, 8, 0) and the wall from (0, 0, 0) to
//! (0, 8, 10), thickened by 1 to the outside of the crease (−z and −x), the gap the moved faces
//! leave at the crease closed by the quarter of the cylinder of radius 1 about it.
use crate::shared::thicken::{creased_sheet, thickened};
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // swept along +y, the floor's normal is +z and the wall's +x: outside is a negative distance
    let sheet = creased_sheet(m)?;
    thickened(m, sheet, -1.0, Join::Arc)
}
