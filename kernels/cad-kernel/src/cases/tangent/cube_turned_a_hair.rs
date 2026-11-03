//! A cube of side 10 from (−5, −5, 0) to (5, 5, 10), united with itself turned 0.001° about z.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;

const HALF: f64 = 5.;
const TURN_DEGREES: f64 = 0.001;

pub fn build() -> Result<Brep, String> {
    let cube = block([-HALF, -HALF, 0.], [HALF, HALF, 2. * HALF])?;
    let turned = cube.moved(&Rigid::turn([0., 0., 0.], [0., 0., 1.], TURN_DEGREES.to_radians()));
    Ok(combined(&cube, &turned, Op::Union, 0.)?)
}
