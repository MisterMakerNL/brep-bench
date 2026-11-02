//! A solid ball of radius 5 centred on the origin.
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    Sphere::new(5.0).build(m)
}
