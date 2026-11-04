//! The box from the origin to (10, 8, 6) shelled inward to a wall 4 thick, open at its top face,
//! z = 6: the walls at y = 0 and y = 8 meet in the plane y = 4, the cavity is flat, and the result
//! is the box itself.
use crate::bench::{face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    let top = face_through(&block, [5.0, 4.0, 6.0])?;
    shell(&block, &order(&block)?, &[top], -4.0)?;
    Ok(block)
}
