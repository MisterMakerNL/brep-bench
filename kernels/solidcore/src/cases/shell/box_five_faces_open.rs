//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at every face but
//! its bottom: the bottom moved in is z = 1, run on to the side openings, so the slab from the
//! origin to (10, 8, 1).
use crate::bench::{faces_through_each, order};
use crate::Refusal;
use solidcore_ops::{make_box, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    // the top and the four sides
    let open = faces_through_each(
        &block,
        &[[5.0, 4.0, 6.0], [5.0, 0.0, 3.0], [5.0, 8.0, 3.0], [0.0, 4.0, 3.0], [10.0, 4.0, 3.0]],
    )?;
    shell(&block, &order(&block)?, &open, -1.0)?;
    Ok(block)
}
