//! A torus about the z axis, centre circle of radius R and tube of radius r, with everything below
//! z = h cut away by a block reaching past it; turned by ψ about the axis (θ, φ).
use crate::solids::boolean::cut;
use crate::solids::fuzz::turned_by;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{make_box_brep, make_torus_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let (big, small, h) = (p.num("R")?, p.num("r")?, p.num("h")?);
    let torus = make_torus_brep(self::p(0.0, 0.0, 0.0), self::p(0.0, 0.0, 1.0), big, small)?;
    let s = 2.0 * (big + small);
    let below = make_box_brep(self::p(-s, -s, -2.0 * small), 2.0 * s, 2.0 * s, h + 2.0 * small)?;
    let sliced = cut(&torus, &below)?;
    Ok(turned_by(&sliced, p.num("theta")?, p.num("phi")?, p.num("psi")?)?)
}
