//! A cylinder of radius 5 on the z axis from z = 0 to 10, the circle around its top face
//! chamfered with distance 5 on both faces, its own radius, so the top shrinks to its centre and
//! the chamfer is a cone with its apex at (0, 0, 10).
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::chamfered;
use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cylinder = make_cylinder(topo, 5.0, 10.0)?;
    let rim = edges_where(topo, cylinder, |q| near(q.z(), 10.0))?;
    chamfered(topo, cylinder, &rim, 5.0)
}
