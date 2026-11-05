//! The ball of radius 10 about the origin, its poles on the y axis and its seam through
//! (−10, 0, 0), and the cylinder of radius 5 on the line x = 5, y = 0 from z = −12 to z = 12, its
//! seam on the line x = 0, y = 0: their cut, through Viviani's curve.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::singular::turned;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep_framed, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep_framed(p(0.0, 0.0, 0.0), 10.0, p(0.0, 1.0, 0.0),
                                       Some(p(-1.0, 0.0, 0.0)))?;
    // the kernel's seam on a z-axis cylinder runs along +y; a quarter turn about z puts it at −x
    let cylinder = make_cylinder_brep(p(0.0, 0.0, -12.0), p(0.0, 0.0, 1.0), 5.0, 24.0)?;
    let cylinder = turned(&cylinder, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), FRAC_PI_2,
                          p(5.0, 0.0, 0.0))?;
    Ok(cut(&ball, &cylinder)?)
}
