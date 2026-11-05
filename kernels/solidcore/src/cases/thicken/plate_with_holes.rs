//! The square in the xy plane from (−10, −10, 0) to (10, 10, 0) with two round holes of radius 3
//! about (±5, 0, 0), a sheet of one flat face, thickened by 1 to each side: a plate 2 thick
//! holed right through.
use crate::bench::{circle, polygon};
use crate::shared::thicken::thickened;
use crate::Refusal;
use solidcore_ops::sweep::planar_faces;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let mut loops = polygon(&[[-10.0, -10.0, 0.0], [10.0, -10.0, 0.0], [10.0, 10.0, 0.0], [-10.0, 10.0, 0.0]]);
    for x in [-5.0, 5.0] {
        loops.push(circle([x, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0], 3.0));
    }
    let sheet = planar_faces(&loops)?;
    // 2 in all, moved back by half first: 1 to each side
    thickened(sheet, 2.0, true)
}
