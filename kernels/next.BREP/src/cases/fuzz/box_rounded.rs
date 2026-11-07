//! A box from the origin to (a, b, c), all twelve of its edges rounded to radius ρ in one fillet,
//! each selected by its middle; turned by ψ about the axis (θ, φ).
use crate::solids::fuzz::turned_by;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid, Vec3};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let (a, b, c) = (p.num("a")?, p.num("b")?, p.num("c")?);
    let block = make_box_brep(self::p(0.0, 0.0, 0.0), a, b, c)?;
    let mut middles: Vec<Vec3> = Vec::new();
    for (u, v) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)] {
        middles.extend([self::p(a / 2.0, u * b, v * c), self::p(u * a, b / 2.0, v * c),
                        self::p(u * a, v * b, c / 2.0)]);
    }
    let rounded = fillet_edges(&block, &middles, None, p.num("rho")?, false, None)?;
    Ok(turned_by(&rounded, p.num("theta")?, p.num("phi")?, p.num("psi")?)?)
}
