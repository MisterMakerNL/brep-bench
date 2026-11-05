//! The ball of radius 5 about the origin, its poles on the z axis, less everything with x < 0: the
//! cut runs along a meridian from pole to pole.
use crate::solids::p;
use crate::solids::singular::split_toward;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(split_toward(&ball, p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0))?)
}
