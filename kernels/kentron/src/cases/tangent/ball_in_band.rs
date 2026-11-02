//! A ball of radius 5 centred on the origin, united with a cylinder of the same radius on the z
//! axis from z = −3 to z = 3: the band's wall touches the ball's equator, tangent to it, so the
//! union is the band with a spherical cap of height 2 standing on each end.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(5.0).build(m)?;
    let band = Cylinder::new(5.0, 6.0).with_frame(frame(p(0.0, 0.0, -3.0), Direction::Z)).build(m)?;
    boolean::union(m, ball, band)
}
