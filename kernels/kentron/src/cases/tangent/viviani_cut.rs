//! A ball of radius 10 about the origin, bored by a solid cylinder of radius 5 about the line
//! x = 5, y = 0, from z = −12 to z = 12, whose wall touches the sphere from inside at (10, 0, 0):
//! what is left is pinched to that point. The ball's poles are on the y axis and its seam through
//! (−10, 0, 0); the cylinder's seam is on the z axis, all away from the touching point.
use crate::shared::tangent::viviani;
use kentron::boolean;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (ball, cylinder) = viviani(m)?;
    boolean::difference(m, ball, cylinder)
}
