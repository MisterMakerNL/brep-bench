//! A ball of radius R about (cx, cy, cz), bored right through along the line through its centre
//! with direction (θ, φ) by a hole of radius r: the drill a cylinder from 2R before the centre to
//! 2R past it.
use crate::solids::boolean::cut;
use crate::solids::fuzz::direction;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let (big, small) = (p.num("R")?, p.num("r")?);
    let centre = self::p(p.num("cx")?, p.num("cy")?, p.num("cz")?);
    let along = direction(p.num("theta")?, p.num("phi")?);
    let ball = make_sphere_brep(centre, big, self::p(0.0, 0.0, 1.0))?;
    let drill = make_cylinder_brep(centre.sub(along.scale(2.0 * big)), along, small, 4.0 * big)?;
    Ok(cut(&ball, &drill)?)
}
