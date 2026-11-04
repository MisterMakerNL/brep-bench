//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at its top face,
//! z = 6: the box less the box from (1, 1, 1) to (9, 7, 6), the textbook shell.
use crate::bench::{face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    // open at the top, z = 6
    let top = face_through(&block, [5.0, 4.0, 6.0])?;
    shell(&block, &order(&block)?, &[top], -1.0)?;
    Ok(block)
}
