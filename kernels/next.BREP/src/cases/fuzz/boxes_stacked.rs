//! A box from the origin to (a1, b1, c1), joined by a box a2 by b2 by c2 standing on its top face
//! with its least corner at (x2, y2, c1); turned by ψ about the axis (θ, φ).
use crate::solids::boolean::union;
use crate::solids::fuzz::turned_by;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let c1 = p.num("c1")?;
    let lower = make_box_brep(self::p(0.0, 0.0, 0.0), p.num("a1")?, p.num("b1")?, c1)?;
    let upper = make_box_brep(self::p(p.num("x2")?, p.num("y2")?, c1), p.num("a2")?, p.num("b2")?,
        p.num("c2")?)?;
    let joined = union(&lower, &upper)?;
    Ok(turned_by(&joined, p.num("theta")?, p.num("phi")?, p.num("psi")?)?)
}
