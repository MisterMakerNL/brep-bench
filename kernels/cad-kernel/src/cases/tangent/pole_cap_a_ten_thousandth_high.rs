//! A ball of radius 5 at the origin in common with the half-space z ≥ 4.9999: its cap 10⁻⁴ high,
//! the rim a circle of radius about 0.0316 around the pole at (0, 0, 5).
use cad_kernel::build::{kept, sphere};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    // the ball, its poles on the z axis: the cap's rim runs around the upper one
    let ball = sphere([0., 0., 0.], [0., 0., 1.], RADIUS)?;
    kept(&ball, [0., 0., RADIUS - HEIGHT], [0., 0., 1.], 0.)
}
