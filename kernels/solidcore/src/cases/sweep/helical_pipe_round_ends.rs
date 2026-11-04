//! A pipe of circular section, radius 1, following the helix of radius 10 about the z axis that
//! rises 5 per turn, from (10, 0, 0), right-handed, for three whole turns, its section square to
//! the path; each end closed by a half ball of radius 1.
use crate::bench::fuse_all;
use crate::shared::sweep::helical_pipe;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let pipe = helical_pipe()?;
    // three whole turns end where they start, 15 higher; the ball about each end's centre
    // closes it, its half inside the pipe lost to the union
    let start = make_sphere([10.0, 0.0, 0.0], 1.0)?;
    let end = make_sphere([10.0, 0.0, 15.0], 1.0)?;
    fuse_all([pipe, start, end])
}
