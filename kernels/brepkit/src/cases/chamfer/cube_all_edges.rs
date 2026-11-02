//! A cube of side 10 from the origin, all twelve edges chamfered in one chamfer with distance 2 on
//! both faces of each, which brepkit closes at each corner with a flat triangle through the three
//! chamfers' ends.
use brepkit_operations::primitives::make_box;
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::chamfered;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let edges = solid_edges(topo, cube)?;
    chamfered(topo, cube, &edges, 2.0)
}
