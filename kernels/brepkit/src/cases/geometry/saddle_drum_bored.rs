//! The drum of `geometry/saddle-drum`, bored through along its axis. The saddle is brepkit's NURBS
//! surface on the case's four control points (degree 1 each way); the block under it over the
//! square −10 ≤ x, y ≤ 10 is that face, bounded by its four straight edges, and the flat faces
//! from them down to the floor z = 0, sewn. The drum is what that block has in common with a
//! cylinder of radius 8 about the z axis, and the bore is a cylinder of radius 3 about it cut
//! away; both cylinders run from below the floor to above the saddle.
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::block_under_spline;
use crate::solids::{common, cut, moved, p};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let corners = vec![
        vec![p(-10.0, -10.0, 22.5), p(-10.0, 10.0, -2.5)],
        vec![p(10.0, -10.0, -2.5), p(10.0, 10.0, 22.5)],
    ];
    let linear = [0.0, 0.0, 1.0, 1.0];
    let block = block_under_spline(topo, (1, 1), &linear, &linear, corners)?;
    let drum = make_cylinder(topo, 8.0, 30.0)?;
    let drum = moved(topo, drum, 0.0, 0.0, -5.0)?;
    let bore = make_cylinder(topo, 3.0, 30.0)?;
    let bore = moved(topo, bore, 0.0, 0.0, -5.0)?;
    let drum = common(topo, block, drum)?;
    Ok(cut(topo, drum, bore)?)
}
