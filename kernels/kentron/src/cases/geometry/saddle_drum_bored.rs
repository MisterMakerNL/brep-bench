//! The drum of `geometry/saddle-drum` (radius 8, floor z = 0, top the bilinear B-spline saddle
//! z = 10 + xy/8) bored through along its axis with radius 3: every point with
//! 9 ≤ x² + y² ≤ 64 and 0 ≤ z ≤ 10 + xy/8. The bore is a cylinder from z = −1 past the saddle.
use crate::bench::{frame, p};
use crate::shared::geometry::saddle;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let column = saddle(m)?;
    let drum = Cylinder::new(8.0, 30.0).build(m)?;
    let drum = boolean::intersection(m, column, drum)?;
    let drill = Cylinder::new(3.0, 30.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    boolean::difference(m, drum, drill)
}
