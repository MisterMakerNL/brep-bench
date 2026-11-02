//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1: every point within 1 of
//! it, each rim the outer quarter of a torus's tube.
use kentron::offset::{self, Join};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    offset::shape(m, cylinder, 1.0, Join::Arc)
}
