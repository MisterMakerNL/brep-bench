//! A pipe of circular section, radius 1, along the helix of radius 10 about the z axis rising 5
//! a turn, from (10, 0, 0), right-handed, three whole turns, the section square to the path,
//! each end closed by a half ball of radius 1: the pipe and a ball about each end of the helix.
use crate::bench::fuse_all;
use crate::shared::sweep::helical_pipe;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    // the helix ends three turns up, 15 above its start, at (10, 0, 15)
    fuse_all([helical_pipe()?, make_sphere([10.0, 0.0, 0.0], 1.0)?, make_sphere([10.0, 0.0, 15.0], 1.0)?])
}
