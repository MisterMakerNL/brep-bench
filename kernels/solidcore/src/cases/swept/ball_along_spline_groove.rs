//! A block, the box from (−6, −6, −8) to (36, 26, 0), grooved by a ball of radius 3 whose centre
//! follows the cubic Bézier (0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0) in the top face:
//! everything the ball passes through cut from the block.
//!
//! solidcore has no sweep of a solid, so the ball's sweep is built as what it is: the tube about
//! its centre's path, the disk of radius 3 square to the curve at its start (along +x) swept
//! along it turning with it, with the ball at each end.
use crate::bench::{circle, cut_all};
use crate::shared::spline::{cubic, swept};
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-6.0, -6.0, -8.0], [42.0, 32.0, 8.0])?;
    let path = cubic([[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [20.0, 20.0, 0.0], [30.0, 20.0, 0.0]]);
    let tube = swept(&[circle([0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0)], path)?;
    let ends = [make_sphere([0.0; 3], 3.0)?, make_sphere([30.0, 20.0, 0.0], 3.0)?];
    cut_all(block, [tube].into_iter().chain(ends))
}
