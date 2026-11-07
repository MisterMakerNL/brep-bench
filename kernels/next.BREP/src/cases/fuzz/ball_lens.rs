//! Balls of radius r1 about the origin and r2 about d·(θ, φ): the lens common to both.
use crate::solids::boolean::common;
use crate::solids::fuzz::direction;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let up = self::p(0.0, 0.0, 1.0);
    let first = make_sphere_brep(self::p(0.0, 0.0, 0.0), p.num("r1")?, up)?;
    let centre = direction(p.num("theta")?, p.num("phi")?).scale(p.num("d")?);
    let second = make_sphere_brep(centre, p.num("r2")?, up)?;
    Ok(common(&first, &second)?)
}
