//! A solid torus about the z axis, its tube of radius 3 about a middle circle of radius 10, shrunk
//! by 1.5: the torus about the same circle whose tube has radius 1.5.
use kentron::offset::{self, Join};
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let torus = Torus::new(10.0, 3.0).build(m)?;
    offset::shape(m, torus, -1.5, Join::Arc)
}
