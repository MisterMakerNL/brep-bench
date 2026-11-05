//! The bent sheet, a floor, a quarter-cylinder bend of radius 2 and an upright wall meeting
//! tangentially, thickened by 1 to the outside of the bend (below the floor, beyond the wall):
//! a bracket 1 thick, its bend's radii 2 and 3.
use crate::shared::thicken::{bent_sheet, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // against the sheet's normals, which point to the inside of the bend
    thickened(bent_sheet()?, -1.0, false)
}
