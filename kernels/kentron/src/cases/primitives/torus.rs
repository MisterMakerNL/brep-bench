//! A solid torus centred on the origin, about the z axis: the circle through the middle of its
//! tube has radius 10, and the tube has radius 2.
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    Torus::new(10.0, 2.0).build(m)
}
