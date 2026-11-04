//! A cube of side 10 between the origin and (10, 10, 10), with a saw cut a millionth wide down
//! into it from the top: the block between x = 5 and x = 5.000001, from z = 5 up, cut from it,
//! the block running past the cube's front, back and top.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let saw = make_box([5.0, -1.0, 5.0], [1e-6, 12.0, 6.0])?;
    cut(cube, saw)
}
