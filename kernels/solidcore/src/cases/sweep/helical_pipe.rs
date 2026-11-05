//! A pipe of circular section, radius 1, along the helix of radius 10 about the z axis rising 5
//! a turn, from (10, 0, 0), right-handed, three whole turns, the section square to the path,
//! the ends flat.
use crate::shared::sweep::helical_pipe;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    helical_pipe()
}
