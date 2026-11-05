//! The angle sheet, a floor (the rectangle in the xy plane from the origin to (10, 8, 0)) and a
//! wall (the rectangle in the yz plane from the origin to (0, 8, 10)) sharing the crease along
//! y, thickened by 1 to the outside of the crease (−z and −x), the gap the moved faces leave at
//! the crease closed by the quarter cylinder of radius 1 about it.
//!
//! The kernel's shell refuses to round the moved faces where they part (`SmApiShellBrepFull`
//! with `bDoExtendedOffset` off rejects the sheet as invalid input), so the angle is thickened
//! with the moved faces extended to meet, at the edge along y at (−1, y, −1), and that edge is
//! rounded to radius 1: the round's axis is the crease, 1 from both moved faces.
use crate::shared::thicken::{angle_sheet, rounded_at, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the floor's normal is +z and the wall's +x: against them
    let sharp = thickened(angle_sheet()?, -1.0, false)?;
    rounded_at(&sharp, &[[-1.0, 4.0, -1.0]], 1.0)
}
