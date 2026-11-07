//! Balls of radius 5.511 about the origin and 3.317 about (4.231, 3.467, 2.653): the lens common to
//! both.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_sphere_brep(p(0.0, 0.0, 0.0), 5.511, p(0.0, 0.0, 1.0))?;
    let second = make_sphere_brep(p(4.231, 3.467, 2.653), 3.317, p(0.0, 0.0, 1.0))?;
    Ok(common(&first, &second)?)
}
