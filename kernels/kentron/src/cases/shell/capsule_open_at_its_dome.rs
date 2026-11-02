//! A capsule's end (the cylinder of radius 5 on the z axis from z = 0 to z = 10, capped by the half
//! ball of radius 5 centred on (0, 0, 10)) shelled inward to a wall 1 thick, open at its
//! hemisphere. The side, moved 1 inward, runs on past its face up into the dome until it meets
//! the sphere at z = 13; the bottom moves to z = 1. The rim is the zone of the sphere between
//! z = 10 and z = 13.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let capsule = shell::capsule_end(m)?;
    // open at the hemisphere, picked by its pole
    shell::inward(m, capsule, &[p(0.0, 0.0, 15.0)], 1.0, Join::Arc)
}
