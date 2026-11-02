//! The block from (−15, −15, 0) to (15, 15, 20) bored along z by a hole of radius 6, its wall
//! drafted 3° about z = 0 so that it widens as it rises.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, drafted};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 6.;
const DRAFT_DEGREES: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let solid = block([-15., -15., 0.], [15., 15., 20.])?;
    // the drill, a cone, runs past both faces: from z = −1, where its radius is 6 − tan 3°, so
    // that it is 6 at the floor, up to z = 21, tapered out of its material as it rises
    let t = DRAFT_DEGREES.to_radians().tan();
    let drill = drafted(&disc([0., 0., -1.], [0., 0., 1.], RADIUS - t), 0., 22., -DRAFT_DEGREES.to_radians())?;
    Ok(combined(&solid, &drill, Op::Cut, 0.)?)
}
