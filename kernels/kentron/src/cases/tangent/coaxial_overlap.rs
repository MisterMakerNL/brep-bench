//! Two cylinders of radius 5 on the z axis, the first from z = 0 to z = 10 and the second from
//! z = 6 to z = 16, united: one cylinder from z = 0 to z = 16, its wall one smooth surface. The
//! second is turned a quarter turn about the z axis (its frame's x along y), so the coincident
//! walls' seams are not in the same place.
use crate::bench::{d, p};
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Cylinder::new(5.0, 10.0).build(m)?;
    let turned = Frame::new(p(0.0, 0.0, 6.0), Direction::Z, d(0.0, 1.0, 0.0));
    let second = Cylinder::new(5.0, 10.0).with_frame(turned).build(m)?;
    boolean::union(m, first, second)
}
