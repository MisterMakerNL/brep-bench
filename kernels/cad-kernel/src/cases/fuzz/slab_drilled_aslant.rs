//! A slab from (−W/2, −D/2, 0) to (W/2, D/2, T), drilled r along the line through (0, 0, T/2)
//! tilted β from the vertical toward the azimuth φ.
use crate::sketch::{block, direction};
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{scale, sub};

pub fn build(p: &Params) -> Result<Brep, String> {
    let (w, d, t) = (p.num("W")?, p.num("D")?, p.num("T")?);
    let slab = block([-w / 2., -d / 2., 0.], [w / 2., d / 2., t])?;
    let along = direction(p.num("beta")?, p.num("phi")?);
    // the drill, a disc square to the hole's axis swept along it, reaching past the slab both ways
    let reach = w + d + t;
    let start = sub([0., 0., t / 2.], scale(along, reach));
    let drill = prism(&disc(start, along, p.num("r")?), 0., 2. * reach)?;
    Ok(combined(&slab, &drill, Op::Cut, 0.)?)
}
