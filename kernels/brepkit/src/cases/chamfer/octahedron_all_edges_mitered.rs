//! The octahedron |x| + |y| + |z| ≤ 10, the convex hull of its six corners, its twelve edges
//! chamfered one after another, each its own chamfer with distance √6 on both faces, ending on the
//! faces at its ends (the chamfers already made among them), so the four chamfers at each corner
//! mitre in a point.
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{chamfered_one_by_one, octahedron};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let solid = octahedron(topo)?;
    let edges = solid_edges(topo, solid)?;
    chamfered_one_by_one(topo, solid, &edges, 6f64.sqrt())
}
