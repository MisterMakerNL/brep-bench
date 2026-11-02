//! A stepped shaft on the z axis, made one solid: a cylinder of radius 5 from z = 0 to z = 10 and,
//! standing on it, a cylinder of radius 3 from z = 10 to z = 20.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let lower = Cylinder::new(5.0, 10.0).build(m)?;
    let upper = Cylinder::new(3.0, 10.0).with_frame(frame(p(0.0, 0.0, 10.0), Direction::Z)).build(m)?;
    boolean::union(m, lower, upper)
}
