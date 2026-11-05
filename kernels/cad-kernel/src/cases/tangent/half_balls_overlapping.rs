//! The halves z ≥ 0 and x ≥ 0 of the ball of radius 5 at the origin, united; the first's poles on
//! the z axis and the second's on the x axis, as the case asks.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{kept, sphere};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;

/// The half of the ball on the side of `axis`, its poles on `axis`.
fn half(axis: [f64; 3]) -> Result<Brep, String> {
    kept(&sphere([0., 0., 0.], axis, RADIUS)?, [0., 0., 0.], axis, 0.)
}

pub fn build() -> Result<Brep, String> {
    Ok(combined(&half([0., 0., 1.])?, &half([1., 0., 0.])?, Op::Union, 0.)?)
}
