//! A capsule's end (the cylinder of radius 5 on the z axis from z = 0 to z = 10, capped by the half
//! ball of radius 5 centred on (0, 0, 10)) shelled inward to a wall 1 thick, open at its base, the
//! disc at z = 0: the capsule less the capsule of radius 4 from z = 0, the moved side and dome
//! still meeting tangentially.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let capsule = shell::capsule_end(m)?;
    // open at the base, z = 0
    shell::inward(m, capsule, &[p(0.0, 0.0, 0.0)], 1.0, Join::Arc)
}
