//! A torus about z of radii 10 and 3 united with the slab (−13, −13, −1)–(13, 13, 1), whose four
//! sides its outer equator touches at their middles.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::torus;
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_4;

const MAJOR: f64 = 10.;
const MINOR: f64 = 3.;
const HALF_SIDE: f64 = 13.;
const HALF_HEIGHT: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    // a torus about z is seamed toward +x, through the point (13, 0, 0) where it touches the slab:
    // turned 45° about z, its seam is clear of all four points of contact
    let torus = torus([0., 0., 0.], up, MAJOR, MINOR)?.moved(&Rigid::turn([0., 0., 0.], up, FRAC_PI_4));
    let slab = block([-HALF_SIDE, -HALF_SIDE, -HALF_HEIGHT], [HALF_SIDE, HALF_SIDE, HALF_HEIGHT])?;
    Ok(combined(&torus, &slab, Op::Union, 0.)?)
}
