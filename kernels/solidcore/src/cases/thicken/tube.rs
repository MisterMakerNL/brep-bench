//! The whole cylinder of radius 5 about the z axis from z = 0 to z = 10, a sheet of one face
//! closing around on itself, thickened by 1 away from the axis: the tube between radii 5 and 6.
use crate::bench::circle;
use crate::shared::thicken::thickened;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let rim = circle([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 5.0);
    let sheet = extrude(&[rim], [0.0, 0.0, 1.0], 10.0, 0.0, DraftCorner::Curvature, false)?;
    // the extruded face's normal points away from the axis
    thickened(sheet, 1.0, false)
}
