//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick with no opening: the box less
//! the box from (1, 1, 1) to (9, 7, 5), one solid with a closed cavity. A shell with no face
//! opened.
use crate::bench::order;
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    shell(&block, &order(&block)?, &[], -1.0)?;
    Ok(block)
}
