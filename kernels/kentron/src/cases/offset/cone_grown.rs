//! A cone on the z axis, its base of radius 3 at z = 0 and its apex at (0, 0, 4), grown by 1:
//! every point within 1 of it, its base's rim a piece of a torus and its apex a cap of a ball.
use kentron::offset::{self, Join};
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cone = Cone::new(3.0, 0.0, 4.0).build(m)?;
    offset::shape(m, cone, 1.0, Join::Arc)
}
