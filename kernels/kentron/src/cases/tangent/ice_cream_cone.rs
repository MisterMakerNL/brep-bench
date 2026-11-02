//! A ball of radius 15 centred on the origin, united with a solid cone on the z axis whose apex
//! is at (0, 0, −25) and whose base is the disc of radius 12 at z = −9. The cone's side is
//! tangent to the sphere along its base's rim, so the union is an ice-cream cone.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cone, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(15.0).build(m)?;
    let cone = Cone::new(0.0, 12.0, 16.0).with_frame(frame(p(0.0, 0.0, -25.0), Direction::Z)).build(m)?;
    boolean::union(m, ball, cone)
}
