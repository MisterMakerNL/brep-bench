//! The rectangle in the xy plane from the origin to (10, 8, 0), a sheet of one flat face,
//! thickened by 2 to its +z side: the box from the origin to (10, 8, 2).
use crate::bench::polygon;
use crate::shared::thicken::thickened;
use crate::Refusal;
use solidcore_ops::sweep::planar_faces;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // counter-clockwise seen from +z, so the face's normal is +z
    let sheet = planar_faces(&polygon(&[[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 8.0, 0.0], [0.0, 8.0, 0.0]]))?;
    thickened(sheet, 2.0, false)
}
