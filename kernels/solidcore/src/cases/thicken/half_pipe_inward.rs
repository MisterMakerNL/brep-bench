//! The half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to z = 10, a
//! sheet of one curved face, thickened by 2 toward the axis: the half tube between radii 3 and 5.
use crate::shared::thicken::{half_pipe, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the extruded face's normal points away from the axis: against it
    thickened(half_pipe()?, -2.0, false)
}
