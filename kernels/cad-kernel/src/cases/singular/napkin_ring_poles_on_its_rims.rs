//! A ball of radius 5 at the origin, bored right through along z by a hole of radius 3; the ball's
//! poles on the line along (3, 0, 4), one on each rim of the hole, as the case asks.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, sphere};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HOLE_RADIUS: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    // the ball, its poles at (3, 0, 4) and (−3, 0, −4)
    let ball = sphere([0., 0., 0.], [3., 0., 4.], RADIUS)?;
    // a drill reaching past the ball both ways along z
    let drill = prism(&disc([0., 0., -2. * RADIUS], [0., 0., 1.], HOLE_RADIUS), 0., 4. * RADIUS)?;
    Ok(combined(&ball, &drill, Op::Cut, 0.)?)
}
