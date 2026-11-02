//! The box from the origin to (10, 8, 6) grown by 1, its edges and corners rounded.
//!
//! brepkit's rounded offset (`offset_solid_arc_v2`) does not build its arc joints yet, so the box
//! is offset with sharp joints, every face moved 1 outward and extended to meet, and the grown
//! box's twelve edges are then rounded to radius 1 in one fillet: each round lies 1 from both
//! moved faces, on the cylinder of radius 1 about the box's own edge, and the three rounds at a
//! corner close on the ball of radius 1 about the box's corner.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::make_box;
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 10.0, 8.0, 6.0)?;
    let grown = offset_solid_v2(topo, block, 1.0)?;
    let edges = solid_edges(topo, grown)?;
    Ok(all_blended(fillet_v2(topo, grown, &edges, 1.0)?)?)
}
