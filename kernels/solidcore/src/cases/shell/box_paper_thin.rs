//! The box from the origin to (10, 8, 6) shelled inward to a wall 0.001 thick, open at its top
//! face, z = 6: the box less the box from (0.001, 0.001, 0.001) to (9.999, 7.999, 6).
use crate::bench::{face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    let top = face_through(&block, [5.0, 4.0, 6.0])?;
    shell(&block, &order(&block)?, &[top], -0.001)?;
    Ok(block)
}
