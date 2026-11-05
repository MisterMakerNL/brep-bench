//! The block from (−10, −10, 0) to (10, 10, 10), united with the solid paraboloid x² + y² ≤ 8z,
//! z ≤ 12.5, whose vertex touches the block's bottom face from inside at the origin.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_line, make_parabola, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    // P(t) = (4t, 2t²) in the frame (axis +z, latus +x): t = 2.5 reaches x = 10, z = 12.5
    let section = [
        make_parabola(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), 2.0, 0.0, 2.5)?,
        make_line(p(10.0, 0.0, 12.5), p(0.0, 0.0, 12.5))?,
        make_line(p(0.0, 0.0, 12.5), p(0.0, 0.0, 0.0))?,
    ];
    let paraboloid = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    let block = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 10.0)?;
    Ok(union(&block, &paraboloid)?)
}
