//! A cylinder of radius 5 on the z axis, from z = 0 to z = 10, the circular edge around its top
//! chamfered with the same distance 5 on both faces, the cylinder's own radius: the top shrinks
//! to its centre and the chamfer is the cone from the circle of radius 5 at z = 5 up to its apex
//! at (0, 0, 10).
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    // (A point on the rim away from the side's seam.)
    let rim = edge_through(m, cylinder, p(0.0, 5.0, 10.0))?;
    Ok(blend::chamfer(m, cylinder, &[rim], 5.0)?)
}
