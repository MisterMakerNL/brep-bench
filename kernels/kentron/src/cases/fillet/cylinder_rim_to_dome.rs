//! A cylinder of radius 5 on the z axis, from z = 0 to z = 10, the circular edge around its top
//! face rounded to radius 5, the cylinder's own: the top shrinks to a point and the round is the
//! hemisphere of radius 5 about (0, 0, 5).
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    let rim = edge_through(m, cylinder, p(0.0, 5.0, 10.0))?;
    Ok(blend::fillet(m, cylinder, &[rim], 5.0)?)
}
