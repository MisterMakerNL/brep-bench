//! A cube of side 10 from (-5, -5, 0) to (5, 5, 10), united with a copy of itself turned 0.001°
//! about the z axis.
use crate::bench::{fuse, rotated};
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([-5.0, -5.0, 0.0], [10.0, 10.0, 10.0])?;
    let turned = rotated(cube.copy()?, [0.0; 3], [0.0, 0.0, 1.0], 0.001)?;
    fuse(cube, turned)
}
