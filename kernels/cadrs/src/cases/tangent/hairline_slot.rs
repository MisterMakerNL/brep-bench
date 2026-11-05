//! A cube of side 10 between the origin and (10, 10, 10), with a saw cut a millionth wide down
//! into it from the top: the slot between x = 5 and x = 5.000001, through the cube in y, from
//! z = 5 up through the top. It is cut with a block running past the cube's front, back and top.
use crate::bench::{block, cut, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let saw = block(k, p3(5.0, -1.0, 5.0), p3(5.000001, 11.0, 11.0))?;
    cut(k, cube, saw)
}
