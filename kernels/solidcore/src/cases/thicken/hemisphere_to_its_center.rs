//! The upper half of the sphere of radius 5 about the origin, a sheet of one face, thickened by
//! 5 toward its center, its whole radius: the half ball of radius 5, the moved face shrunk to
//! the origin.
use crate::shared::thicken::{hemisphere, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the turned face's normal points away from the center: against it
    thickened(hemisphere()?, -5.0, false)
}
