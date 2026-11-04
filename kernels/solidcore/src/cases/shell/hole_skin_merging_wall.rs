//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z
//! about x = 15.5, y = 10, 1.5 from the side x = 20, shelled inward to a wall 1 thick, open at its
//! top face, z = 10: the tube round the bore, of radius 4, runs into the inside wall x = 19 and
//! merges with it.
use crate::bench::{cut, face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [20.0, 20.0, 10.0])?;
    // the drill runs past both faces
    let drill = make_cylinder([15.5, 10.0, -1.0], 3.0, 12.0)?;
    let drilled = cut(block, drill)?;
    let top = face_through(&drilled, [2.0, 2.0, 10.0])?;
    shell(&drilled, &order(&drilled)?, &[top], -1.0)?;
    Ok(drilled)
}
