//! A torus about the z axis, centred on the origin, its tube's centre circle of radius 10 and its
//! tube of radius 2, with everything where x < 0 cut away: the half-space bounded by the plane
//! x = 0 on the side of negative x.
use crate::bench::p;
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let torus = Torus::new(10.0, 2.0).build(m)?;
    let negative_x = half_space(m, p(0.0, 0.0, 0.0), Direction::X, p(-1.0, 0.0, 0.0))?;
    Ok(boolean::difference(m, torus, negative_x)?)
}
