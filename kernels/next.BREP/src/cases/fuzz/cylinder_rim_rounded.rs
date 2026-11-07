//! A cylinder of radius R on the z axis from z = 0 to H, the circular edge around its top rounded to
//! ρ, the edge selected by a point on it off the seam; turned by ψ about the axis (θ, φ).
use crate::solids::fillet::on_circle;
use crate::solids::fuzz::turned_by;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let (radius, height) = (p.num("R")?, p.num("H")?);
    let cylinder = make_cylinder_brep(self::p(0.0, 0.0, 0.0), self::p(0.0, 0.0, 1.0), radius, height)?;
    let rounded = fillet_edges(&cylinder, &[on_circle(radius, height, 1.0)], None, p.num("rho")?,
        false, None)?;
    Ok(turned_by(&rounded, p.num("theta")?, p.num("phi")?, p.num("psi")?)?)
}
