//! A cube of side 10 from (−5, −5, −10) to (5, 5, 0) in common with the half-space above the plane
//! z = y·10⁻⁶: a wedge 5·10⁻⁶ thick at y = −5, thinning to an edge along the x axis.
use crate::sketch::block;
use cad_kernel::build::kept;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const HALF: f64 = 5.;
const SLOPE: f64 = 1e-6;

pub fn build() -> Result<Brep, String> {
    let cube = block([-HALF, -HALF, -2. * HALF], [HALF, HALF, 0.])?;
    // the side of the plane through the x axis its normal (0, −SLOPE, 1) points to
    kept(&cube, [0., 0., 0.], unit([0., -SLOPE, 1.]), 0.)
}
