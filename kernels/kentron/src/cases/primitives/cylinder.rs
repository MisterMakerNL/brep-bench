//! A solid cylinder of radius 5 and height 10 whose axis is the z axis, standing on the xy plane:
//! it runs from z = 0 to z = 10.
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    Cylinder::new(5.0, 10.0).build(m)
}
