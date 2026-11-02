//! A cube of side 10, all twelve edges rounded to radius 1 in one fillet.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::primitives::make_box;
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let edges = solid_edges(topo, cube)?;
    Ok(all_blended(fillet_v2(topo, cube, &edges, 1.0)?)?)
}
