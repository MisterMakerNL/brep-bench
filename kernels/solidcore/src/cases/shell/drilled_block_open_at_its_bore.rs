//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z
//! about x = 10, y = 10, shelled inward to a wall 1 thick, open at the bore: a hollow box, its
//! cavity from (1, 1, 1) to (19, 19, 9) opening into the hole.
use crate::bench::{cut, face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [20.0, 20.0, 10.0])?;
    // the drill runs past both faces
    let drill = make_cylinder([10.0, 10.0, -1.0], 3.0, 12.0)?;
    let drilled = cut(block, drill)?;
    let bore = face_through(&drilled, [13.0, 10.0, 5.0])?;
    shell(&drilled, &order(&drilled)?, &[bore], -1.0)?;
    Ok(drilled)
}
