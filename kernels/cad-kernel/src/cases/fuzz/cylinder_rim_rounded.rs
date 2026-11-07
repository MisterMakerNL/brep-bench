//! A cylinder of radius R on z from z = 0 to H, the rim of its top face rounded to ρ; turned by ψ
//! about the axis (θ, φ).
use crate::sketch::turned_by;
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

pub fn build(p: &Params) -> Result<Brep, String> {
    let (radius, height) = (p.num("R")?, p.num("H")?);
    let cylinder = prism(&disc([0., 0., 0.], [0., 0., 1.], radius), 0., height)?;
    // the rim is picked by a point on it; the kernel rolls the ball round the loop where the top
    // meets the side and builds the ring of material outside the round
    let round = roll(&cylinder, [0., radius, height], p.num("rho")?, tolerance(cylinder.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    let rounded = combined(&cylinder, &round.piece, op, 0.)?;
    Ok(turned_by(&rounded, p.num("theta")?, p.num("phi")?, p.num("psi")?))
}
