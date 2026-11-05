//! Two halves of the solid torus about the z axis (middle circle of radius 10, tube of radius 3),
//! united: the half with z ≥ 0, and the half with x ≥ 0 of a copy turned a quarter turn about z,
//! so that the faces that coincide are not split in the same places.
use crate::solids::boolean::{turned_about_z, union};
use crate::solids::p;
use crate::solids::singular::keep_toward;
use crate::Refusal;
use brep_kernel::{make_torus_brep, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let o = p(0.0, 0.0, 0.0);
    let torus = make_torus_brep(o, p(0.0, 0.0, 1.0), 10.0, 3.0)?;
    let upper = keep_toward(&torus, o, p(0.0, 0.0, 1.0), 20.0)?;
    let front = keep_toward(&turned_about_z(&torus, FRAC_PI_2)?, o, p(1.0, 0.0, 0.0), 20.0)?;
    Ok(union(&upper, &front)?)
}
