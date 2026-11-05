//! A cube of side 10 at the origin united with a cylinder of radius 5 on the x axis from x = −10 to
//! 10, whose wall touches four of the cube's faces along their middle lines.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_4;

const HALF: f64 = 5.;
const RADIUS: f64 = 5.;
const REACH: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let cube = block([-HALF; 3], [HALF; 3])?;
    // a disc about x is seamed toward +y, on the line where the wall touches the face y = 5:
    // turned 45° about x, the seam runs along y = z, clear of the four lines of contact
    let cylinder = prism(&disc([-REACH, 0., 0.], [1., 0., 0.], RADIUS), 0., 2. * REACH)?
        .moved(&Rigid::turn([0., 0., 0.], [1., 0., 0.], FRAC_PI_4));
    Ok(combined(&cube, &cylinder, Op::Union, 0.)?)
}
