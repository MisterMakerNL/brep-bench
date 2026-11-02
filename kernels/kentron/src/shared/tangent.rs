//! What the tangent implementations share.
use crate::bench::p;
use kentron::math::{Direction, Frame};
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

/// Viviani's ball and cylinder: the ball of radius 10 about the origin, its poles on the y axis
/// and its seam through (−10, 0, 0), and the solid cylinder of radius 5 about the line x = 5,
/// y = 0, from z = −12 to z = 12, its seam on the z axis. The cylinder's wall touches the sphere
/// from inside at (10, 0, 0).
pub fn viviani(m: &mut Model) -> kentron::Result<(Shape, Shape)> {
    let away = Direction::new(-1.0, 0.0, 0.0);
    let ball = Sphere::new(10.0).with_frame(Frame::new(p(0.0, 0.0, 0.0), Direction::Y, away)).build(m)?;
    let cylinder =
        Cylinder::new(5.0, 24.0).with_frame(Frame::new(p(5.0, 0.0, -12.0), Direction::Z, away)).build(m)?;
    Ok((ball, cylinder))
}
