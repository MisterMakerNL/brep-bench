//! The halves z ≥ 0 and x ≥ 0 of the solid torus about z of radii 10 and 3, united; the second's
//! seam a quarter turn about z from the first's.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{kept, torus};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;

const MAJOR: f64 = 10.;
const MINOR: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let (origin, up) = ([0., 0., 0.], [0., 0., 1.]);
    // a torus about z is seamed toward +x; the second's turned a quarter about z, toward +y
    let first = torus(origin, up, MAJOR, MINOR)?;
    let second = first.moved(&Rigid::turn(origin, up, FRAC_PI_2));
    let upper = kept(&first, origin, up, 0.)?;
    let front = kept(&second, origin, [1., 0., 0.], 0.)?;
    Ok(combined(&upper, &front, Op::Union, 0.)?)
}
