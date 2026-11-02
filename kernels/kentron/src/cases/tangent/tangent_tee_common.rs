//! Two solid cylinders crossing at right angles: the first, radius 5, along the x axis from
//! x = −15 to x = 15; the second, radius 3, parallel to the y axis from y = −15 to y = 15 on the
//! line x = 0, z = 2. Their tops touch at (0, 0, 5), tangent there. Their common part.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let along_x = Frame::from_z(p(-15.0, 0.0, 0.0), Direction::X);
    let along_y = Frame::from_z(p(0.0, -15.0, 2.0), Direction::Y);
    let first = Cylinder::new(5.0, 30.0).with_frame(along_x).build(m)?;
    let second = Cylinder::new(3.0, 30.0).with_frame(along_y).build(m)?;
    boolean::intersection(m, first, second)
}
