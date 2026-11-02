//! The octahedron |x| + |y| + |z| ≤ 10, the convex hull of its six corners, all twelve edges
//! chamfered in one chamfer with distance √6 on both faces of each, which brepkit closes at each
//! corner with a flat square through the four chamfers' ends.
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{chamfered, octahedron};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let solid = octahedron(topo)?;
    let edges = solid_edges(topo, solid)?;
    chamfered(topo, solid, &edges, 6f64.sqrt())
}
