//! Balls of radius r1 about the origin and r2 about d·(θ, φ): the lens common to both.
use crate::sketch::direction;
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::scale;

pub fn build(p: &Params) -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let first = sphere([0., 0., 0.], up, p.num("r1")?)?;
    let centre = scale(direction(p.num("theta")?, p.num("phi")?), p.num("d")?);
    let second = sphere(centre, up, p.num("r2")?)?;
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
