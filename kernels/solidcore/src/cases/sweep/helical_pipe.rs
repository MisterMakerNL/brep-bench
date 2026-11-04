//! A pipe of circular section, radius 1, following the helix of radius 10 about the z axis that
//! rises 5 per turn, from (10, 0, 0), right-handed, for three whole turns; its section square to
//! the path, its ends flat.
use crate::shared::sweep::helical_pipe;
use solidcore_ops::{Body, Result};

pub fn build() -> Result<Body> {
    helical_pipe()
}
