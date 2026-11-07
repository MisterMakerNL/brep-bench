//! Cylinders of radius r and length 12r centred on the origin, along x and along (cos α, sin α, 0):
//! the solid common to both, turned by ψ about the axis (θ, φ).
use crate::sketch::turned_by;
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::scale;

pub fn build(p: &Params) -> Result<Brep, String> {
    let r = p.num("r")?;
    let (s, c) = p.num("alpha")?.to_radians().sin_cos();
    // each a disc at one end, square to its axis, swept along it
    let cylinder = |axis: [f64; 3]| prism(&disc(scale(axis, -6. * r), axis, r), 0., 12. * r);
    let both = combined(&cylinder([1., 0., 0.])?, &cylinder([c, s, 0.])?, Op::Common, 0.)?;
    Ok(turned_by(&both, p.num("theta")?, p.num("phi")?, p.num("psi")?))
}
