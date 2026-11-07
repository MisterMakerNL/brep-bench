//! A slab from (−W/2, −D/2, 0) to (W/2, D/2, T), drilled by a hole of radius r along the line
//! through (0, 0, T/2) tilted β from the vertical toward the azimuth φ: the drill a cylinder along
//! that line, reaching W + D + T past the slab's middle both ways.
use crate::solids::boolean::cut;
use crate::solids::fuzz::direction;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let (w, d, t) = (p.num("W")?, p.num("D")?, p.num("T")?);
    let slab = make_box_brep(self::p(-w / 2.0, -d / 2.0, 0.0), w, d, t)?;
    let along = direction(p.num("beta")?, p.num("phi")?);
    let reach = w + d + t;
    let start = self::p(0.0, 0.0, t / 2.0).sub(along.scale(reach));
    let drill = make_cylinder_brep(start, along, p.num("r")?, 2.0 * reach)?;
    Ok(cut(&slab, &drill)?)
}
