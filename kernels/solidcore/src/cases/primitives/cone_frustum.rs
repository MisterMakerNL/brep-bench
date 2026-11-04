//! A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base (z = 0) and
//! radius 2 at its top (z = 6).
use solidcore_ops::{make_cone, Body, Result};

pub fn build() -> Result<Body> {
    make_cone([0.0, 0.0, 0.0], 5.0, 2.0, 6.0)
}
