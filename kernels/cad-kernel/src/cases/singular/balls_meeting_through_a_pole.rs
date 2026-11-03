//! Two balls of radius 5, centred on the origin and on (4.8, 0, 3.6), united; each with its poles
//! on the vertical through its centre, so that the circle where they meet passes through a pole of
//! each.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;

/// The ball of radius `RADIUS` about `centre`, its poles on the vertical through it.
fn ball(centre: V) -> Result<Brep, String> {
    sphere(centre, [0., 0., 1.], RADIUS)
}

pub fn build() -> Result<Brep, String> {
    let (first, second) = (ball([0., 0., 0.])?, ball([4.8, 0., 3.6])?);
    Ok(combined(&first, &second, Op::Union, 0.)?)
}
