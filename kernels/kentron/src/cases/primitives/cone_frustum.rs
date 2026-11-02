//! A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base (z = 0) and
//! radius 2 at its top (z = 6).
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    Cone::new(5.0, 2.0, 6.0).build(m)
}
