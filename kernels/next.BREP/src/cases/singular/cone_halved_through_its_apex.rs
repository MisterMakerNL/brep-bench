//! The solid cone with its base the disc of radius 5 about the origin in the xy plane and its apex
//! at (0, 0, 10), less everything with x < 0: the cut holds the axis and the apex.
use crate::solids::p;
use crate::solids::singular::split_toward;
use crate::Refusal;
use brep_kernel::{make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cone = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 0.0, 10.0)?;
    Ok(split_toward(&cone, p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0))?)
}
