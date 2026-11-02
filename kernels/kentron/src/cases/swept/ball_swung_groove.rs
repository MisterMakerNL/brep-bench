//! A block, the box from (−6, −6, −8) to (18, 18, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre swings a quarter turn about the z axis, right-handed, from
//! (12, 0, 0) to (0, 12, 0), staying in the top face: everything the ball passes through, cut.
//!
//! kentron has no sweep of a solid (`sweep::revolve` takes faces, wires, edges and vertices), so
//! the ball's sweep is built as elementary geometry gives it: a point p of the ball, centre c,
//! moving with it has velocity ω × p, which runs along the sphere exactly where it is square to
//! p − c, on the great circle through the axis and the centre. So what the ball passes through is
//! the tube that circle's disc sweeps, turned about the z axis (a quarter of a torus), with the
//! ball where it starts and where it ends.
use crate::bench::p;
use crate::shared::sweep::disc;
use crate::shared::swept::cut_each;
use kentron::math::{Axis, Direction, Frame};
use kentron::primitives::{Cuboid, Sphere};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-6.0, -6.0, -8.0), p(18.0, 18.0, 0.0)).build(m)?;
    // The ball's great circle through the z axis, where it starts: in the plane y = 0.
    let section = disc(m, p(12.0, 0.0, 0.0), Direction::Y, 3.0)?;
    let tube = sweep::revolve(m, section, Axis::new(p(0.0, 0.0, 0.0), Direction::Z), FRAC_PI_2)?.shape;
    let start = Sphere::new(3.0).with_frame(Frame::from_z(p(12.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let end = Sphere::new(3.0).with_frame(Frame::from_z(p(0.0, 12.0, 0.0), Direction::Z)).build(m)?;
    // The tube and each ball cut from the block in turn.
    cut_each(m, block, &[tube, start, end])
}
