//! The solid torus about the z axis (middle circle of radius 10, tube of radius 3), turned 45°
//! about z so that its seam misses the points of contact, united with the slab from
//! (−13, −13, −1) to (13, 13, 1), whose four sides its outer equator touches.
use crate::solids::boolean::{turned_about_z, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_torus_brep, BrepSolid};
use std::f64::consts::FRAC_PI_4;

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 10.0, 3.0)?;
    let torus = turned_about_z(&torus, FRAC_PI_4)?;
    let slab = make_box_brep(p(-13.0, -13.0, -1.0), 26.0, 26.0, 2.0)?;
    Ok(union(&slab, &torus)?)
}
