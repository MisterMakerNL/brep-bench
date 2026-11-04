//! A rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin and the
//! opposite corner at (10, 20, 30).
use solidcore_ops::{make_box, Body, Result};

pub fn build() -> Result<Body> {
    make_box([0.0, 0.0, 0.0], [10.0, 20.0, 30.0])
}
