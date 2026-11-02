//! A solid torus about the z axis, centred on the origin, its tube of radius 2 about a middle
//! circle of radius 6, hollowed to a wall 0.5 thick with no opening: the torus less the torus of
//! tube radius 1.5, one solid bounded by two tori, the inner one facing in.
use crate::shared::shell;
use crate::Refusal;
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let torus = Torus::new(6.0, 2.0).build(m)?;
    // thick_solid given no opening: the moved torus bounds a closed cavity of its own
    shell::hollowed(m, torus, 0.5)
}
