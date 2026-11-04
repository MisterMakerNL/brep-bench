//! The box from the origin to (30, 20, 20), drilled right through along x by a hole of radius 4
//! on the line y = 10, z = 10; then the faces x = 0 and x = 30, where the hole comes out, drafted
//! by 8° about the neutral plane z = 0, pull +z, each leaning inward as it rises.
use crate::bench::{cut, rotated};
use crate::shared::draft::draft_about_z;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [30.0, 20.0, 20.0])?;
    // the drill along x, running past both faces: a cylinder on z turned onto x about y
    let drill = make_cylinder([10.0, 10.0, -5.0], 4.0, 40.0)?;
    let drill = rotated(drill, [10.0, 10.0, 10.0], [0.0, 1.0, 0.0], 90.0)?;
    let drilled = cut(block, drill)?;
    draft_about_z(&drilled, &[[0.0, 2.0, 2.0], [30.0, 2.0, 2.0]], 0.0, [0.0, 0.0, 1.0], 8.0)?;
    Ok(drilled)
}
