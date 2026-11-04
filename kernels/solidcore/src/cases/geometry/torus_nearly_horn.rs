//! A solid torus about the z axis: middle circle radius 5, tube radius 4.99, its hole a circle
//! of radius 0.01 about the axis.
use solidcore_ops::{make_torus, Body, Result};

pub fn build() -> Result<Body> {
    make_torus([0.0, 0.0, 0.0], 5.0, 4.99)
}
