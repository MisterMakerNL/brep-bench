//! Two balls of radius 5, about the origin and about (4.8, 0, 3.6), each with its poles on the
//! vertical through its center: their union, whose circle of contact runs through a pole of each.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let second = make_sphere_brep(p(4.8, 0.0, 3.6), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(union(&first, &second)?)
}
