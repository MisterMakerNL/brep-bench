//! A solid torus centred on the origin, about the z axis: the circle through the middle of its
//! tube has radius 10, and the tube has radius 2.
use solidcore_ops::{make_torus, Body, Result};

pub fn build() -> Result<Body> {
    make_torus([0.0, 0.0, 0.0], 10.0, 2.0)
}
