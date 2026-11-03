//! The box from the origin to (40, 30, 10), drilled right through along z by a hole of radius 5
//! about the vertical line through (20, 15). Its top face, the plane z = 10, is moved up by 6, to
//! z = 16; the four sides and the hole's wall run on up to meet it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, move_faces, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 10.0)?;
    let drill = make_cylinder_brep(p(20.0, 15.0, -1.0), p(0.0, 0.0, 1.0), 5.0, 12.0)?;
    let drilled = cut(&block, &drill)?;
    let top = face_at(&drilled, p(5.0, 5.0, 10.0))?;
    Ok(move_faces(&drilled, &[top], p(0.0, 0.0, 6.0))?)
}
