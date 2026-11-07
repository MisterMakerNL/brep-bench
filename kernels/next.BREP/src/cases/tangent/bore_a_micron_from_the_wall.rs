//! A cylinder of radius 5 on the z axis from z = 0 to 10, less a hole of radius 4 along the line
//! x = 0.999999, y = 0, the drill a cylinder from z = −1 to 11: the wall between them 10⁻⁶ thick at
//! (5, 0, z).
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let body = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let drill = make_cylinder_brep(p(0.999999, 0.0, -1.0), p(0.0, 0.0, 1.0), 4.0, 12.0)?;
    Ok(cut(&body, &drill)?)
}
