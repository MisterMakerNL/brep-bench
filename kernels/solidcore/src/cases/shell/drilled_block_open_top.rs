//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z
//! about x = 10, y = 10, shelled inward to a wall 1 thick, open at its top face, z = 10: a cup with
//! a tube between radii 3 and 4 standing in it round the bore.
use crate::bench::{cut, face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [20.0, 20.0, 10.0])?;
    // the drill runs past both faces
    let drill = make_cylinder([10.0, 10.0, -1.0], 3.0, 12.0)?;
    let drilled = cut(block, drill)?;
    let top = face_through(&drilled, [2.0, 2.0, 10.0])?;
    shell(&drilled, &order(&drilled)?, &[top], -1.0)?;
    Ok(drilled)
}
