//! A ball of radius 5 at the origin, its poles on the z axis, less the cap beyond the plane
//! x + z = 5, which passes through its pole at (0, 0, 5).
use cad_kernel::build::{kept, sphere};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // the ball, its poles on the vertical through its centre
    let ball = sphere([0., 0., 0.], [0., 0., 1.], RADIUS)?;
    // the plane through the pole, kept on the side of the centre: towards −(1, 0, 1)
    kept(&ball, [0., 0., RADIUS], unit([-1., 0., -1.]), 0.)
}
