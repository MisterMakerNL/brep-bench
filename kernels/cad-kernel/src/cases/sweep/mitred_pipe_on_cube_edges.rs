//! A pipe of radius 2 swept along three edges of a cube of side 30, (0, 0, 0) to (30, 0, 0), to
//! (30, 30, 0), to (30, 30, 30), mitred at its two corners in the planes bisecting them; the
//! section a disc square to the path, the ends flat.
use crate::sketch::mitred;
use cad_kernel::build::disc;
use cad_kernel::topo::Brep;

const SIDE: f64 = 30.;
const RADIUS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let path = [[0., 0., 0.], [SIDE, 0., 0.], [SIDE, SIDE, 0.], [SIDE, SIDE, SIDE]];
    mitred(&disc(path[0], [1., 0., 0.], RADIUS), &path, false, RADIUS)
}
