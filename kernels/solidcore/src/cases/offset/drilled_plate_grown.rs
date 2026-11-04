//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 1: every point within 1 of it. The plate grows as a box does, and the hole
//! narrows to radius 2, flaring at its rims into the inner quarters of tori.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([-10.0, -10.0, 0.0], [20.0, 20.0, 4.0])?;
    // the drill runs past both faces
    let drill = make_cylinder([0.0, 0.0, -1.0], 3.0, 6.0)?;
    let drilled = cut(plate, drill)?;
    offset_rounded(&drilled, 1.0)
}
