//! A drum, the cylinder of radius 8 on the z axis from z = 0 to z = 6, with a flange round its
//! side from radius 8 to 11 and from z = 2.2 to z = 3.8, shrunk by 1: every point at least 1 from
//! its outside. The flange, 1.6 thick, shrinks away to a low ridge: two concave pieces of tori
//! meeting in the circle of radius 7.4 at z = 3.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let drum = Cylinder::new(8.0, 6.0).build(m)?;
    let flange = Cylinder::new(11.0, 1.6).with_frame(frame(p(0.0, 0.0, 2.2), Direction::Z)).build(m)?;
    let flanged = boolean::union(m, drum, flange)?;
    offset::shape(m, flanged, -1.0, Join::Arc)
}
