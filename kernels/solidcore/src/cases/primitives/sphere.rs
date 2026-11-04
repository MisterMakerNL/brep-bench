//! A solid ball of radius 5 centred on the origin.
use solidcore_ops::{make_sphere, Body, Result};

pub fn build() -> Result<Body> {
    make_sphere([0.0, 0.0, 0.0], 5.0)
}
