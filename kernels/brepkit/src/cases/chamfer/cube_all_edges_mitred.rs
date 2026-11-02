//! A cube of side 10 from the origin, its twelve edges chamfered one after another, each its own
//! chamfer with distance 2 on both faces, ending on the faces at its ends (the chamfers already
//! made among them), so the three chamfers at each corner mitre in a point.
use brepkit_operations::primitives::make_box;
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::chamfered_one_by_one;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let edges = solid_edges(topo, cube)?;
    chamfered_one_by_one(topo, cube, &edges, 2.0)
}
