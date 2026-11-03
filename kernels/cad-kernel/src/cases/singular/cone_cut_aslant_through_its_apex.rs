//! A cone on z, its base the disc of radius 5 in the xy plane and its apex at (0, 0, 10), cut by
//! the plane 4x + z = 10 through its apex and kept on the side of its axis.
use cad_kernel::build::{cone, kept};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let cone = cone([0., 0., 0.], [0., 0., 1.], RADIUS, 0., HEIGHT)?;
    // the plane through the apex with normal (4, 0, 1), kept where 4x + z < 10
    kept(&cone, [0., 0., HEIGHT], unit([-4., 0., -1.]), 0.)
}
