//! A cube of side 10 centred on the origin and the ball of radius 5√2 about the origin, which
//! touches each of the cube's twelve edges at its midpoint. Their common part: six discs inscribed
//! in the cube's faces and eight three-sided sphere patches over the corners, all meeting at the
//! twelve edge midpoints.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::{Cuboid, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::from_corners(p(-5.0, -5.0, -5.0), p(5.0, 5.0, 5.0)).build(m)?;
    let ball = Sphere::new(5.0 * 2f64.sqrt()).build(m)?;
    boolean::intersection(m, cube, ball)
}
