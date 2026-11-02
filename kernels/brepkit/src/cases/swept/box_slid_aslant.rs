//! The solid a box fills sliding along (8, 5, 3). brepkit sweeps faces, not solids, so it is
//! built by elementary geometry: a convex solid slid along a line without turning fills the
//! convex hull of where it starts and where it ends, so the result is brepkit's convex hull of
//! the corners of the box from (0, 0, 0) to (10, 6, 4) and of that box moved along the slide.
use brepkit_operations::copy::copy_solid;
use brepkit_operations::primitives::{make_box, make_convex_hull};
use brepkit_topology::explorer::solid_vertices;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::moved;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let start = make_box(topo, 10.0, 6.0, 4.0)?;
    let end = copy_solid(topo, start)?;
    let end = moved(topo, end, 8.0, 5.0, 3.0)?;
    let mut corners = Vec::new();
    for solid in [start, end] {
        for vertex in solid_vertices(topo, solid)? {
            corners.push(topo.vertex(vertex)?.point());
        }
    }
    Ok(make_convex_hull(topo, &corners)?)
}
