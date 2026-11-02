//! The L-shaped plate (its outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) shrunk by 1, its convex edges sharp and its concave one rounded.
//!
//! brepkit's rounded offset (`offset_solid_arc_v2`) does not build its arc joints yet, so the
//! plate is offset 1 inward with sharp joints and the shrunk plate's concave edge, along x = 4,
//! y = 4, is then rounded to radius 1: the round lies 1 from both moved faces, on the cylinder of
//! radius 1 about the plate's own concave edge.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::l_block;
use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = l_block(topo)?;
    let shrunk = offset_solid_v2(topo, plate, -1.0)?;
    let concave = edges_where(topo, shrunk, |q| near(q.x(), 4.0) && near(q.y(), 4.0))?;
    Ok(all_blended(fillet_v2(topo, shrunk, &concave, 1.0)?)?)
}
