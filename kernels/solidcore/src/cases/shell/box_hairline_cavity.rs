//! The box from the origin to (10, 8, 6) shelled inward to a wall 3.99 thick, open at its top
//! face, z = 6: the cavity is a slot 0.02 wide, the box from (3.99, 3.99, 3.99) to
//! (6.01, 4.01, 6).
use crate::bench::{face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    let top = face_through(&block, [5.0, 4.0, 6.0])?;
    shell(&block, &order(&block)?, &[top], -3.99)?;
    Ok(block)
}
