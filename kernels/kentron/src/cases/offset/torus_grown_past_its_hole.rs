//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 4, grown
//! by 3: every point within 5 of the middle circle, an apple that closes over the hole, dimpled
//! where it meets the axis at z = ±3.
use kentron::offset::{self, Join};
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let torus = Torus::new(4.0, 2.0).build(m)?;
    offset::shape(m, torus, 3.0, Join::Arc)
}
