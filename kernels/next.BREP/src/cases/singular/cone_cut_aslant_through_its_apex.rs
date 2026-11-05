//! The solid cone with its base the disc of radius 5 about the origin in the xy plane and its apex
//! at (0, 0, 10), less what lies beyond the plane 4x + z = 10, which passes through the apex.
use crate::solids::p;
use crate::solids::singular::split_toward;
use crate::Refusal;
use brep_kernel::{make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cone = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 0.0, 10.0)?;
    // keep 4x + z ≤ 10
    Ok(split_toward(&cone, p(0.0, 0.0, 10.0), p(-4.0, 0.0, -1.0))?)
}
