//! A cone on z, its base the disc of radius 5 in the xy plane and its apex at (0, 0, 10),
//! everything on the side x < 0 cut away by the plane through its axis.
use cad_kernel::build::{cone, kept};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // from the base (radius 5) up to the apex (radius 0)
    let cone = cone([0., 0., 0.], [0., 0., 1.], RADIUS, 0., HEIGHT)?;
    // the part on the side of +x
    kept(&cone, [0., 0., 0.], [1., 0., 0.], 0.)
}
