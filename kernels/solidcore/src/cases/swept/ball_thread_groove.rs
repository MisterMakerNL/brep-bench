//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a ball of
//! radius 1 whose centre follows the helix of radius 5 about the z axis from (5, 0, −2),
//! right-handed, rising 4 a turn, for six whole turns to (5, 0, 22): everything the ball passes
//! through cut from the rod.
//!
//! solidcore has no sweep of a solid, so the ball's sweep is built as what it is: the tube about
//! its centre's path, the disk of radius 1 square to the helix carried along it by the kernel's
//! screw sweep, with the ball at each end.
use crate::bench::cut_all;
use crate::shared::sweep::pipe_along_helix;
use crate::shared::swept::rod;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let tube = pipe_along_helix(1.0, 5.0, -2.0, 4.0, 6.0)?;
    let ends = [make_sphere([5.0, 0.0, -2.0], 1.0)?, make_sphere([5.0, 0.0, 22.0], 1.0)?];
    cut_all(rod()?, [tube].into_iter().chain(ends))
}
