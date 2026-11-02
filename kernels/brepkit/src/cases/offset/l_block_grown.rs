//! The L-shaped plate (its outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) grown by 1, its convex edges rounded and its concave one sharp.
//!
//! brepkit's rounded offset (`offset_solid_arc_v2`) does not build its arc joints yet, so the
//! plate is offset with sharp joints and the grown plate's convex edges, all but the concave
//! one along x = 6, y = 6, are then rounded to radius 1: each round lies 1 from both moved
//! faces, on the cylinder of radius 1 about the plate's own edge, and closes at a convex corner
//! on the ball of radius 1 about the plate's corner.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_topology::explorer::solid_edges;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::l_block;
use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = l_block(topo)?;
    let grown = offset_solid_v2(topo, plate, 1.0)?;
    let concave = edges_where(topo, grown, |q| near(q.x(), 6.0) && near(q.y(), 6.0))?;
    let convex: Vec<_> = solid_edges(topo, grown)?
        .into_iter()
        .filter(|e| !concave.contains(e))
        .collect();
    Ok(all_blended(fillet_v2(topo, grown, &convex, 1.0)?)?)
}
