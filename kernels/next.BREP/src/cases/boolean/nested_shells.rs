//! Balls of radius 10 about the origin, 8 about (1, 0, 0), 6 about (1, 1, 0), 4 about (1, 1, 1) and
//! 2 about (1, 1.5, 1.5): the first less the second, joined with the third less the fourth, joined
//! with the fifth — three bodies, each in the hollow of the one around it.
use crate::solids::boolean::{cut, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid, Vec3};

fn ball(centre: Vec3, radius: f64) -> Result<BrepSolid, String> {
    make_sphere_brep(centre, radius, p(0.0, 0.0, 1.0))
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let outer = cut(&ball(p(0.0, 0.0, 0.0), 10.0)?, &ball(p(1.0, 0.0, 0.0), 8.0)?)?;
    let middle = cut(&ball(p(1.0, 1.0, 0.0), 6.0)?, &ball(p(1.0, 1.0, 1.0), 4.0)?)?;
    let both = union(&outer, &middle)?;
    Ok(union(&both, &ball(p(1.0, 1.5, 1.5), 2.0)?)?)
}
