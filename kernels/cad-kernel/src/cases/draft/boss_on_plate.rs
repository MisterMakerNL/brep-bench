//! The plate from (−20, −20, 0) to (20, 20, 5) with a boss of radius 8 on z from z = 5 to 17, its
//! side drafted 4° inward about z = 5.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, drafted};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = block([-20., -20., 0.], [20., 20., 5.])?;
    // the boss's circle on the plate, swept up with its side tapered from there: a cone
    let boss = drafted(&disc([0., 0., 5.], [0., 0., 1.], 8.), 0., 12., 4f64.to_radians())?;
    Ok(combined(&plate, &boss, Op::Union, 0.)?)
}
