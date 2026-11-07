//! A torus about z of radii R and r, kept above the plane z = h; turned by ψ about the axis (θ, φ).
use crate::sketch::turned_by;
use crate::Params;
use cad_kernel::build::{kept, torus};
use cad_kernel::topo::Brep;

pub fn build(p: &Params) -> Result<Brep, String> {
    let torus = torus([0., 0., 0.], [0., 0., 1.], p.num("R")?, p.num("r")?)?;
    // what lies above z = h: in common with a block standing on that plane
    let sliced = kept(&torus, [0., 0., p.num("h")?], [0., 0., 1.], 0.)?;
    Ok(turned_by(&sliced, p.num("theta")?, p.num("phi")?, p.num("psi")?))
}
