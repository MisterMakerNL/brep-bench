//! A ball of radius 6.528 about (18.676, −15.427, 14.527), bored right through along the line
//! through its centre in the direction (0.448, −0.297, −0.844) by a hole of radius 3.408.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, sphere};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{scale, sub, unit};

const RADIUS: f64 = 6.528;
const HOLE_RADIUS: f64 = 3.408;

pub fn build() -> Result<Brep, String> {
    let centre = [18.676, -15.427, 14.527];
    let along = unit([0.448, -0.297, -0.844]);
    // the ball, its poles on the vertical through its centre
    let ball = sphere(centre, [0., 0., 1.], RADIUS)?;
    // a drill reaching past the ball both ways along the hole's axis
    let drill = prism(&disc(sub(centre, scale(along, 2. * RADIUS)), along, HOLE_RADIUS), 0., 4. * RADIUS)?;
    Ok(combined(&ball, &drill, Op::Cut, 0.)?)
}
