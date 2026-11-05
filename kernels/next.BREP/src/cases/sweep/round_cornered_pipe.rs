//! A pipe of radius 3 along two straight runs, from (−30, 0, 0) to the origin and on to
//! (0, 30, 0), the section carried around the corner by turning it about the z axis through it.
//!
//! next.BREP's sweep along a cornered chain only mitres the corner; it has no round-corner joint.
//! The solid follows from the description by elementary geometry: it is every point within 3 of
//! the path, cut flat at its ends, so it is the union of the two runs' cylinders, each ending flat
//! at the corner, and the ball of radius 3 about the corner. The ball's part with x < 0 lies in the
//! first run's cylinder and its part with y > 0 in the second's, so what it adds is exactly the
//! quarter ball the turning disk fills outside the corner.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_cylinder_brep(p(-30.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 3.0, 30.0)?;
    let second = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 3.0, 30.0)?;
    let corner = make_sphere_brep(p(0.0, 0.0, 0.0), 3.0, p(0.0, 0.0, 1.0))?;
    let runs = union(&first, &second)?;
    Ok(union(&runs, &corner)?)
}
