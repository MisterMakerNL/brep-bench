//! The bent sheet of `thicken/bent-flange`, a floor, a quarter-cylinder bend of radius 2 and an
//! upright wall meeting tangentially, thickened by 3 to the inside of the bend, more than its
//! radius: the bend's offset turns inside out and leaves the solid, the floor's and the wall's
//! meeting at a concave edge instead.
use crate::shared::thicken::{bent_sheet, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // along the sheet's normals, which point to the inside of the bend
    thickened(bent_sheet()?, 3.0, false)
}
