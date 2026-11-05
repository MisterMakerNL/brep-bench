//! A cube of side 10 from (−5, −5, 0) to (5, 5, 10), united with a copy of itself turned 0.001°
//! about the z axis.
use crate::solids::boolean::{turned_about_z, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(-5.0, -5.0, 0.0), 10.0, 10.0, 10.0)?;
    let turned = turned_about_z(&cube, 0.001f64.to_radians())?;
    Ok(union(&cube, &turned)?)
}
