//! The ball of radius 5 about the origin, its poles on the z axis, in common with a block above
//! the plane z = 4.9999 that reaches past the ball on every side but its floor: a cap 10⁻⁴ high
//! about the pole (0, 0, 5).
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let block = make_box_brep(p(-10.0, -10.0, 4.9999), 20.0, 20.0, 10.0)?;
    Ok(common(&ball, &block)?)
}
