//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at its top and
//! bottom faces: a rectangular tube, the box less the box from (1, 1, 0) to (9, 7, 6).
use crate::bench::{faces_through_each, order};
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    let open = faces_through_each(&block, &[[5.0, 4.0, 6.0], [5.0, 4.0, 0.0]])?;
    shell(&block, &order(&block)?, &open, -1.0)?;
    Ok(block)
}
