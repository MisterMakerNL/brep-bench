//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its curved side. Only the two ends are walls, moved 1 inward to z = 1 and z = 9 and
//! running out to the cylinder: two separate discs, from z = 0 to 1 and from z = 9 to 10.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    // open at the curved side, picked by a point off its seam
    shell::inward(m, cylinder, &[p(0.0, 5.0, 5.0)], 1.0, Join::Arc)
}
