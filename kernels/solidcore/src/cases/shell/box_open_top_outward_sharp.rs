//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top face,
//! z = 6, its edges kept sharp: the box from (−1, −1, −1) to (11, 9, 6) less the box. Ops's shell
//! extends the moved faces until they meet.
use crate::bench::{face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    let top = face_through(&block, [5.0, 4.0, 6.0])?;
    shell(&block, &order(&block)?, &[top], 1.0)?;
    Ok(block)
}
