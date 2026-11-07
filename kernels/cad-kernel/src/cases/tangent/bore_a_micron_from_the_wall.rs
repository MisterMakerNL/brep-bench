//! A cylinder of radius 5 on z from z = 0 to 10, bored by a hole of radius 4 along the line
//! x = 0.999999, y = 0: the wall between them 10⁻⁶ thick at (5, 0, z).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let body = prism(&disc([0., 0., 0.], up, 5.), 0., 10.)?;
    // the drill from z = −1 to 11, past both ends
    let drill = prism(&disc([0.999999, 0., -1.], up, 4.), 0., 12.)?;
    Ok(combined(&body, &drill, Op::Cut, 0.)?)
}
