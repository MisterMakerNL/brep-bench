//! Two solid cylinders of radius 5 and length 30 crossing at right angles: the first along the x
//! axis from x = −15 to x = 15, the second parallel to the y axis from y = −15 to y = 15 on the
//! line x = 0, z = 9.99. Their common part, a cushion only 0.01 thick.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let along_x = Frame::from_z(p(-15.0, 0.0, 0.0), Direction::X);
    let along_y = Frame::from_z(p(0.0, -15.0, 9.99), Direction::Y);
    let first = Cylinder::new(5.0, 30.0).with_frame(along_x).build(m)?;
    let second = Cylinder::new(5.0, 30.0).with_frame(along_y).build(m)?;
    boolean::intersection(m, first, second)
}
