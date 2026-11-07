//! A box from the origin to (a1, b1, c1), with a box a2 by b2 by c2 standing on it from (x2, y2,
//! c1), joined; turned by ψ about the axis (θ, φ).
use crate::sketch::{block, turned_by};
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build(p: &Params) -> Result<Brep, String> {
    let (c1, x2, y2) = (p.num("c1")?, p.num("x2")?, p.num("y2")?);
    let lower = block([0., 0., 0.], [p.num("a1")?, p.num("b1")?, c1])?;
    let upper = block([x2, y2, c1], [x2 + p.num("a2")?, y2 + p.num("b2")?, c1 + p.num("c2")?])?;
    let joined = combined(&lower, &upper, Op::Union, 0.)?;
    Ok(turned_by(&joined, p.num("theta")?, p.num("phi")?, p.num("psi")?))
}
