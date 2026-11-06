//! A square column from z = 0 to z = 40, its section at height z the square of side 30 − z/2
//! centred on the z axis and turned about it, right-handed, by 4.5° for every unit of height, its
//! corners on four conical spirals, its walls ruled by horizontal segments between them.
//!
//! The section both turns and shrinks in proportion to the height risen, which is the kernel's
//! twisted sweep with a scaling law (`sweep_profile_twisted_scaled`): the square of side 30 at
//! z = 0 swept up the z axis, turning half a turn about it and scaled to a third (side 10) at the
//! top, both in proportion to the height risen.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_line, sweep_profile_twisted_scaled, BrepSolid};
use std::f64::consts::PI;

pub fn build() -> Result<BrepSolid, Refusal> {
    let foot = polygon(&[p(15.0, -15.0, 0.0), p(15.0, 15.0, 0.0), p(-15.0, 15.0, 0.0),
        p(-15.0, -15.0, 0.0)])?;
    let axis = make_line(p(0.0, 0.0, 0.0), p(0.0, 0.0, 40.0))?;
    Ok(sweep_profile_twisted_scaled(&foot, &axis, PI, 1.0 / 3.0, None)?)
}
