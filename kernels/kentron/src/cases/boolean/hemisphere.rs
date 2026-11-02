//! A ball of radius 5 centred on the origin, with everything below the xy plane cut away: the
//! half-space bounded by the plane z = 0 on the side of negative z.
use crate::bench::p;
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let ball = Sphere::new(5.0).build(m)?;
    let below = half_space(m, p(0.0, 0.0, 0.0), Direction::Z, p(0.0, 0.0, -1.0))?;
    Ok(boolean::difference(m, ball, below)?)
}
