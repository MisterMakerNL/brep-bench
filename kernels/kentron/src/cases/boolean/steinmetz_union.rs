//! Two cylinders of radius 5 and length 30 crossing at right angles, the first along the x axis
//! from x = −15 to 15, the second along the y axis from y = −15 to 15: their union.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let along_x = Cylinder::new(5.0, 30.0).with_frame(frame(p(-15.0, 0.0, 0.0), Direction::X)).build(m)?;
    let along_y = Cylinder::new(5.0, 30.0).with_frame(frame(p(0.0, -15.0, 0.0), Direction::Y)).build(m)?;
    boolean::union(m, along_x, along_y)
}
