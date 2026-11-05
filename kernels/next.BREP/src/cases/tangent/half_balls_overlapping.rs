//! Two halves of the ball of radius 5 about the origin, united: the half with z ≥ 0 of a ball
//! whose poles are on the z axis, and the half with x ≥ 0 of one whose poles are on the x axis.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::singular::keep_toward;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let o = p(0.0, 0.0, 0.0);
    let upper = keep_toward(&make_sphere_brep(o, 5.0, p(0.0, 0.0, 1.0))?, o, p(0.0, 0.0, 1.0), 20.0)?;
    let front = keep_toward(&make_sphere_brep(o, 5.0, p(1.0, 0.0, 0.0))?, o, p(1.0, 0.0, 0.0), 20.0)?;
    Ok(union(&upper, &front)?)
}
