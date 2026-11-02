//! Two balls of radius 5, one about the origin and one about (9.999999, 0, 0), overlapping to a
//! depth of only 10⁻⁶. Their common part, a lens a millionth thick.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(5.0).build(m)?;
    let second = Sphere::new(5.0).with_frame(frame(p(9.999999, 0.0, 0.0), Direction::Z)).build(m)?;
    boolean::intersection(m, first, second)
}
