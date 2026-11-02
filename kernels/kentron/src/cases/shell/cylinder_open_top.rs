//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its top face: a cup, the cylinder less the cylinder of radius 4 from z = 1 to z = 10.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    // open at the top, z = 10
    shell::inward(m, cylinder, &[p(0.0, 0.0, 10.0)], 1.0, Join::Arc)
}
