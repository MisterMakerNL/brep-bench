//! The block from (−15, −15, 0) to (15, 15, 20) bored along z by a hole of radius 6, its wall
//! drafted 3° about z = 0 so that it widens as it rises.
use crate::sketch::{block, drafted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::disc;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let solid = block([-15., -15., 0.], [15., 15., 20.])?;
    // the drill, a cone through the hole's circle at the floor, the neutral plane, tapered out of
    // its material as it rises (a negative draft); it runs past both faces, from z = −1 to 21
    let drill = drafted_through(&disc([0., 0., 0.], [0., 0., 1.], 6.), 1., 21., -3f64.to_radians())?;
    Ok(combined(&solid, &drill, Op::Cut, 0.)?)
}
