//! Cylinders of radius r and length 12r centred on the origin, along x and along (cos α, sin α, 0):
//! the solid common to both, turned by ψ about the axis (θ, φ).
use crate::solids::boolean::common;
use crate::solids::fuzz::turned_by;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let r = p.num("r")?;
    let (s, c) = p.num("alpha")?.to_radians().sin_cos();
    let along_x = make_cylinder_brep(self::p(-6.0 * r, 0.0, 0.0), self::p(1.0, 0.0, 0.0), r, 12.0 * r)?;
    let aslant = make_cylinder_brep(self::p(-6.0 * r * c, -6.0 * r * s, 0.0), self::p(c, s, 0.0), r,
        12.0 * r)?;
    let both = common(&along_x, &aslant)?;
    Ok(turned_by(&both, p.num("theta")?, p.num("phi")?, p.num("psi")?)?)
}
