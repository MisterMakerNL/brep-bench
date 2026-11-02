//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3), the
//! convex hull of its corners, grown by 1 with its edges and corners rounded.
//!
//! brepkit's rounded offset (`offset_solid_arc_v2`) does not build its arc joints yet, so the
//! tetrahedron is offset with sharp joints, each face moved 1 outward and extended to meet, and
//! the grown tetrahedron's six edges are then rounded to radius 1 in one fillet: each round lies
//! 1 from both moved faces, on the cylinder of radius 1 about the tetrahedron's own edge, and the
//! three rounds at a corner close on the ball of radius 1 about its corner.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::make_convex_hull;
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let corners = [
        p(3.0, 3.0, 3.0),
        p(3.0, -3.0, -3.0),
        p(-3.0, 3.0, -3.0),
        p(-3.0, -3.0, 3.0),
    ];
    let tetrahedron = make_convex_hull(topo, &corners)?;
    let grown = offset_solid_v2(topo, tetrahedron, 1.0)?;
    let edges = solid_edges(topo, grown)?;
    Ok(all_blended(fillet_v2(topo, grown, &edges, 1.0)?)?)
}
