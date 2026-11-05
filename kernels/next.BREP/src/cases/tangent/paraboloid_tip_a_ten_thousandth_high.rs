//! The solid paraboloid x² + y² ≤ 8z, z ≤ 8 (as `geometry/paraboloid`) in common with a block
//! under the plane z = 10⁻⁴ that reaches past the paraboloid on every side but its top: the tip.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_line, make_parabola, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

const H: f64 = 1e-4;

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = [
        make_parabola(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), 2.0, 0.0, 2.0)?,
        make_line(p(8.0, 0.0, 8.0), p(0.0, 0.0, 8.0))?,
        make_line(p(0.0, 0.0, 8.0), p(0.0, 0.0, 0.0))?,
    ];
    let paraboloid = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    let block = make_box_brep(p(-10.0, -10.0, -1.0), 20.0, 20.0, 1.0 + H)?;
    Ok(common(&paraboloid, &block)?)
}
