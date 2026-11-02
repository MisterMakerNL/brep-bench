//! A ball of radius 5 centred on the origin, hollowed to a wall 1 thick with no opening: the ball
//! less the ball of radius 4, one solid bounded by two spheres, the inner one facing in.
use crate::shared::shell;
use crate::Refusal;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let ball = Sphere::new(5.0).build(m)?;
    // thick_solid given no opening: the moved sphere bounds a closed cavity of its own
    shell::hollowed(m, ball, 1.0)
}
