//! The box from the origin to (40, 30, 8), drilled right through along z by a hole of radius 5
//! about the vertical line through (20, 15). The hole's wall is offset 2 into the plate, away from
//! its axis, to the cylinder of radius 7; the top and the bottom are trimmed back to meet it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, offset_ruled_face, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 8.0)?;
    let drill = make_cylinder_brep(p(20.0, 15.0, -1.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let drilled = cut(&plate, &drill)?;
    let wall = face_at(&drilled, p(25.0, 15.0, 4.0))?;
    // The kernel pushes a face along its outward normal, out of the material and into the hole,
    // toward its axis; a negative distance pushes it the other way, into the plate.
    Ok(offset_ruled_face(&drilled, wall, -2.0)?)
}
