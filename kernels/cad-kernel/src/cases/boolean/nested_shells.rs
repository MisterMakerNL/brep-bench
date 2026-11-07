//! Balls of radius 10 about the origin, 8 about (1, 0, 0), 6 about (1, 1, 0), 4 about (1, 1, 1) and
//! 2 about (1, 1.5, 1.5): the first less the second, joined with the third less the fourth, joined
//! with the fifth.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

/// The ball of radius `r` about `centre`, its poles on the vertical through it.
fn ball(centre: V, r: f64) -> Result<Brep, String> {
    sphere(centre, [0., 0., 1.], r)
}

pub fn build() -> Result<Brep, String> {
    let outer = combined(&ball([0., 0., 0.], 10.)?, &ball([1., 0., 0.], 8.)?, Op::Cut, 0.)?;
    let middle = combined(&ball([1., 1., 0.], 6.)?, &ball([1., 1., 1.], 4.)?, Op::Cut, 0.)?;
    let both = combined(&outer, &middle, Op::Union, 0.)?;
    Ok(combined(&both, &ball([1., 1.5, 1.5], 2.)?, Op::Union, 0.)?)
}
