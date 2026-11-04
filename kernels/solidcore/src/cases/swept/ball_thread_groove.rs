//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! cut round it by a ball of radius 1 whose centre follows the helix of radius 5 about the z axis
//! from (5, 0, -2), right-handed, rising 4 a turn, for six whole turns to (5, 0, 22): everything
//! the ball passes through, cut from the rod.
use crate::bench::{cut, fuse_all};
use crate::shared::sweep::helix_pipe;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let rod = make_cylinder([0.0; 3], 5.0, 20.0)?;
    // The kernel sweeps a whole body only along a line or about an axis
    // (`shared::swept`), not along a helix, so the ball's sweep is built as elementary geometry
    // gives it: the tube of the ball's radius about its centre's path (the disc square to the
    // helix carried by the helix's screw), with the ball at each end.
    let tube = helix_pipe([5.0, 0.0, -2.0], 5.0, 4.0, 6.0, 1.0)?;
    let start = make_sphere([5.0, 0.0, -2.0], 1.0)?;
    let end = make_sphere([5.0, 0.0, 22.0], 1.0)?;
    let groove = fuse_all([tube, start, end])?;
    cut(rod, groove)
}
