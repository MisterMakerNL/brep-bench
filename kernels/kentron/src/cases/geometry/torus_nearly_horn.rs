//! A solid torus about the z axis whose hole is nearly closed: the middle circle of its tube has
//! radius 5 and the tube radius 4.99.
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    Torus::new(5.0, 4.99).build(m)
}
