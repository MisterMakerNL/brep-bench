//! A solid cylinder of radius 5 and height 10 whose axis is the z axis, standing on the xy plane:
//! it runs from z = 0 to z = 10.
use solidcore_ops::{make_cylinder, Body, Result};

pub fn build() -> Result<Body> {
    make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)
}
