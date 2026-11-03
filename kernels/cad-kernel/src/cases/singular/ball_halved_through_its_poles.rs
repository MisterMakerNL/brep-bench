//! A ball of radius 5 at the origin, everything on the side x < 0 cut away; its poles on the z
//! axis, in the cutting plane, as the case asks.
use cad_kernel::build::{kept, sphere};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let centre = [0., 0., 0.];
    // the ball, its poles on the z axis: both in the yz plane
    let ball = sphere(centre, [0., 0., 1.], RADIUS)?;
    // the part on the side of +x
    kept(&ball, centre, [1., 0., 0.], 0.)
}
